//! ゲーム画面のUI構築

use bevy::prelude::*;
use common::{consts::BG_SURFACE, resources::GameAssets};

use crate::components::{action::GameButtonAction, screen::OnGameScreen};
use crate::layer::Layer;
use crate::rendering::{spawn_cell_highlight, spawn_grid_sprite};
use crate::resources::world::World;
use crate::systems::button_handler::*;
use crate::systems::ui::*;

/// ゲーム画面の全UIを構築するシステム
///
/// ボトムパネル（操作ボタン群）とワールド（セルグリッドスプライト＋ハイライト）を生成する。
pub fn spawn_screen(
    mut commands: Commands,
    world: Res<World>,
    game_assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
) {
    // NOTE: Bottom Panel
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.),
                padding: UiRect::horizontal(Val::Px(12.)),
                ..default()
            },
            Layer::BottomPanel.as_render_layer(),
            OnGameScreen,
            BackgroundColor(BG_SURFACE),
        ))
        .with_children(|p| {
            // Generation counter
            spawn_generation_text(p, &game_assets, world.generation_count);

            // Simulation controls: Start / Stop / Next
            p.spawn(button_group_node()).with_children(|p| {
                spawn_action_button(p, &game_assets, "Start", GameButtonAction::Start)
                    .observe(handle_start)
                    .observe(handle_over)
                    .observe(handle_out);
                spawn_action_button(p, &game_assets, "Stop", GameButtonAction::Stop)
                    .observe(handle_stop)
                    .observe(handle_over)
                    .observe(handle_out);
                spawn_action_button(p, &game_assets, "Next", GameButtonAction::Next)
                    .observe(handle_next)
                    .observe(handle_over)
                    .observe(handle_out);
            });

            // Reset / Clear
            p.spawn(button_group_node()).with_children(|p| {
                spawn_action_button(p, &game_assets, "Reset", GameButtonAction::Reset)
                    .observe(handle_reset)
                    .observe(handle_over)
                    .observe(handle_out);
                spawn_action_button(p, &game_assets, "Clear", GameButtonAction::Clear)
                    .observe(handle_clear)
                    .observe(handle_over)
                    .observe(handle_out);
            });

            // Grid toggle
            p.spawn(button_group_node()).with_children(|p| {
                spawn_action_button(p, &game_assets, "Grid", GameButtonAction::GridToggle)
                    .observe(handle_grid_toggle)
                    .observe(handle_over)
                    .observe(handle_out);
            });

            // Speed control
            p.spawn(stepper_row_node()).with_children(|p| {
                spawn_small_button(p, &game_assets, "<", GameButtonAction::SpeedDown)
                    .observe(handle_speed_down)
                    .observe(handle_over)
                    .observe(handle_out);
                spawn_stepper_label(p, &game_assets, "Speed");
                spawn_small_button(p, &game_assets, ">", GameButtonAction::SpeedUp)
                    .observe(handle_speed_up)
                    .observe(handle_over)
                    .observe(handle_out);
            });

            // Zoom control
            p.spawn(stepper_row_node()).with_children(|p| {
                spawn_small_button(p, &game_assets, "<", GameButtonAction::ZoomDown)
                    .observe(handle_zoom_down)
                    .observe(handle_over)
                    .observe(handle_out);
                spawn_stepper_label(p, &game_assets, "Zoom");
                spawn_small_button(p, &game_assets, ">", GameButtonAction::ZoomUp)
                    .observe(handle_zoom_up)
                    .observe(handle_over)
                    .observe(handle_out);
            });
        });

    // NOTE: World
    spawn_grid_sprite(&mut commands, &mut images, &world);
    spawn_cell_highlight(&mut commands);
}
