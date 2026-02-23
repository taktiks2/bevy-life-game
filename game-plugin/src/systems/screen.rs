//! ゲーム画面のUI構築

use bevy::prelude::*;
use common::{
    consts::{BG_SURFACE, SPACING_MD, SPACING_SM},
    resources::GameAssets,
    ui::handle_screen_button_out,
};

use crate::components::{action::GameButtonAction, screen::OnGameScreen, slider::SliderKind};
use crate::layer::Layer;
use crate::rendering::spawn_cell_highlight;
use crate::resources::world::World;
use crate::systems::button_handler::*;
use crate::systems::ui::*;

/// ゲーム画面の全UIを構築するシステム
///
/// ボトムパネル（操作ボタン群）とセルハイライトを生成する。
/// チャンクスプライトは `manage_chunks` システムが動的に管理する。
pub fn spawn_screen(mut commands: Commands, world: Res<World>, game_assets: Res<GameAssets>) {
    // NOTE: Bottom Panel
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(SPACING_SM),
                padding: UiRect::horizontal(Val::Px(SPACING_MD)),
                ..default()
            },
            Layer::BottomPanel.as_render_layer(),
            OnGameScreen,
            BackgroundColor(BG_SURFACE),
        ))
        .with_children(|p| {
            // Generation counter
            spawn_generation_text(p, &game_assets, world.generation_count);

            // Simulation controls: Start/Stop toggle + Next
            p.spawn(button_group_node()).with_children(|p| {
                spawn_action_button(p, &game_assets, "Start", GameButtonAction::ToggleSimulation)
                    .observe(handle_toggle_simulation)
                    .observe(handle_over)
                    .observe(handle_screen_button_out);
                spawn_action_button(p, &game_assets, "Next", GameButtonAction::Next)
                    .observe(handle_next)
                    .observe(handle_over)
                    .observe(handle_screen_button_out);
            });

            // Reset / Clear
            p.spawn(button_group_node()).with_children(|p| {
                spawn_action_button(p, &game_assets, "Reset", GameButtonAction::Reset)
                    .observe(handle_reset)
                    .observe(handle_over)
                    .observe(handle_screen_button_out);
                spawn_action_button(p, &game_assets, "Clear", GameButtonAction::Clear)
                    .observe(handle_clear)
                    .observe(handle_over)
                    .observe(handle_screen_button_out);
            });

            // Grid toggle
            p.spawn(button_group_node()).with_children(|p| {
                spawn_action_button(p, &game_assets, "Grid", GameButtonAction::GridToggle)
                    .observe(handle_grid_toggle)
                    .observe(handle_over)
                    .observe(handle_screen_button_out);
            });

            // Speed slider
            spawn_slider(p, &game_assets, "Speed", SliderKind::Speed);

            // Zoom slider
            spawn_slider(p, &game_assets, "Zoom", SliderKind::Zoom);
        });

    // NOTE: Cell highlight (チャンクスプライトはmanage_chunksが管理)
    spawn_cell_highlight(&mut commands);
}
