use bevy::{color::palettes::css::*, prelude::*};
use common::resources::GameAssets;

use crate::components::{action::GameButtonAction, screen::OnGameScreen};
use crate::layer::Layer;
use crate::resources::world::World;
use crate::systems::action::*;
use crate::systems::ui::*;

pub fn spawn_screen(
    mut commands: Commands,
    world: Res<World>,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // NOTE: Side Menu
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            Layer::SideMenu.as_render_layer(),
            OnGameScreen,
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|p| {
            spawn_generation_text(p, &game_assets, world.generation_count);

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
            spawn_action_button(p, &game_assets, "Reset", GameButtonAction::Reset)
                .observe(handle_reset)
                .observe(handle_over)
                .observe(handle_out);
            spawn_action_button(p, &game_assets, "Clear", GameButtonAction::Clear)
                .observe(handle_clear)
                .observe(handle_over)
                .observe(handle_out);

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
    spawn_cell_grid(&mut commands, &world, &mut meshes, &mut materials);
}
