//! ゲームプラグイン
//!
//! コンウェイのライフゲームのメイン画面を提供する。
//! サイドメニュー（操作パネル）とワールド（セルグリッド）の2カメラ構成で描画する。

use bevy::{camera::Viewport, prelude::*};
use common::{
    consts::{INITIAL_CAMERA_SCALE, MAIN_PHYSICAL_WIDTH, PHYSICAL_HEIGHT, SUB_PHYSICAL_WIDTH},
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
    camera::{SideMenuCamera, WorldCamera},
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
                setup_side_menu_camera,
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
                despawn_entity::<SideMenuCamera>,
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
            (world_clear, play_audios).run_if(in_state(GameState::Game)),
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

/// サイドメニュー用カメラを生成する
///
/// ウィンドウ左側20%をビューポートとし、操作ボタン群を描画する。
/// order=1 でワールドカメラより上に描画される。
pub fn setup_side_menu_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            // NOTE: 複数のカメラを使う場合、優先順位を付ける必要がある
            order: 1,
            viewport: Some(Viewport {
                physical_position: [0, 0].into(),
                physical_size: [SUB_PHYSICAL_WIDTH, PHYSICAL_HEIGHT].into(),
                ..default()
            }),
            ..default()
        },
        SideMenuCamera,
    ));
}

/// ワールド描画用カメラを生成する
///
/// ウィンドウ右側80%をビューポートとし、セルグリッドを描画する。
/// OrthographicProjectionによるズーム・パン操作に対応する。
pub fn setup_world_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            // NOTE: 複数のカメラを使う場合、優先順位を付ける必要がある
            order: 0,
            viewport: Some(Viewport {
                physical_position: [SUB_PHYSICAL_WIDTH, 0].into(),
                physical_size: [MAIN_PHYSICAL_WIDTH, PHYSICAL_HEIGHT].into(),
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
