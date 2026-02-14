//! ゲームプラグイン
//!
//! コンウェイのライフゲームのメイン画面を提供する。
//! ボトムパネル（操作パネル）とワールド（セルグリッド）の2カメラ構成で描画する。

use bevy::{camera::Viewport, prelude::*};
use common::{
    consts::{INITIAL_CAMERA_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH, calc_viewport_sizes},
    resources::GameAssets,
    states::GameState,
    systems::despawn_entity,
};

mod components;
mod events;
mod layer;
mod rendering;
mod resources;
mod states;
mod systems;

use components::{
    camera::{BottomPanelCamera, WorldCamera},
    screen::OnGameScreen,
};
use events::*;
use layer::Layer;
use resources::{
    timer::{SimulationTimer, SpaceKeyTimer},
    world::World,
};
use states::SimulationState;
use resources::interaction::{AudioCooldown, HoveredCell};
use systems::{
    audio::play_audios,
    cell_operations::*,
    grid::{handle_grid_click, update_cell_highlight},
    input::*,
    screen::spawn_screen,
    viewport::update_camera_viewports,
};

/// ゲーム画面のBevyプラグイン
///
/// カメラ設定・リソース初期化・UI構築・入力処理・シミュレーションの
/// 全システムを登録する。
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                setup_bottom_panel_camera,
                setup_world_camera,
                setup_resource,
                spawn_screen,
            )
                .chain(),
        );
        app.add_systems(
            OnExit(GameState::Game),
            (
                despawn_entity::<OnGameScreen>,
                despawn_entity::<BottomPanelCamera>,
                despawn_entity::<WorldCamera>,
            ),
        );
        app.add_systems(
            Update,
            (
                update_cells,
                game_input_keyboard_handling,
                game_input_zoom_handling,
                progress_generation,
                progress_generation_trigger.run_if(in_state(SimulationState::Simulating)),
                update_generation,
                reset_generation,
                handle_grid_click,
                update_cell_highlight,
            )
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            (world_clear, play_audios, update_camera_viewports)
                .run_if(in_state(GameState::Game)),
        );
        app.insert_resource(SpaceKeyTimer::new());
        app.init_resource::<HoveredCell>();
        app.init_resource::<AudioCooldown>();
        app.init_state::<SimulationState>();
        app.add_message::<ProgressGenerationEvent>();
        app.add_message::<GenerationResetEvent>();
        app.add_message::<WorldClearEvent>();
        app.add_message::<PlayAudioEvent>();
    }
}

/// ボトムパネル用カメラを生成する
///
/// ウィンドウ下部をビューポートとし、操作ボタン群を描画する。
/// order=1 でワールドカメラより上に描画される。
pub fn setup_bottom_panel_camera(mut commands: Commands, windows: Query<&Window>) {
    let (pw, ph) = windows
        .single()
        .map(|w| (w.physical_width(), w.physical_height()))
        .unwrap_or((WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));
    let sizes = calc_viewport_sizes(pw, ph);

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            viewport: Some(Viewport {
                physical_position: [0, sizes.main_height].into(),
                physical_size: [sizes.viewport_width, sizes.panel_height].into(),
                ..default()
            }),
            ..default()
        },
        BottomPanelCamera,
    ));
}

/// ワールド描画用カメラを生成する
///
/// ウィンドウ上部をビューポートとし、セルグリッドを描画する。
/// OrthographicProjectionによるズーム・パン操作に対応する。
pub fn setup_world_camera(mut commands: Commands, windows: Query<&Window>) {
    let (pw, ph) = windows
        .single()
        .map(|w| (w.physical_width(), w.physical_height()))
        .unwrap_or((WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));
    let sizes = calc_viewport_sizes(pw, ph);

    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            viewport: Some(Viewport {
                physical_position: [0, 0].into(),
                physical_size: [sizes.viewport_width, sizes.main_height].into(),
                ..default()
            }),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scale: INITIAL_CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
        WorldCamera,
        Layer::World.as_render_layer(),
    ));
}

/// Worldリソースとシミュレーションタイマーを初期化する
fn setup_resource(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.insert_resource(World::new(
        game_assets.world_width,
        game_assets.world_height,
    ));
    commands.insert_resource(SimulationTimer::new(game_assets.tick_interval));
}
