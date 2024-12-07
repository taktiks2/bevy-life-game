use bevy::prelude::*;
use common::{resources::GameAssets, states::GameState, systems::despawn_entity};

mod components;
mod events;
mod layer;
mod resources;
mod states;
mod systems;

use components::screen::OnGameScreen;
use events::*;
use layer::Layer;
use resources::{
    timer::{SimulationTimer, SpaceKeyTimer},
    world::World,
};
use states::SimulationState;
use systems::{
    action::game_action, cell_operations::*, input::game_input_keyboard_handling,
    screen::spawn_screen,
};

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
                switch_cell_state,
                update_cells,
                game_input_keyboard_handling,
                progress_generation,
                game_action,
                progress_generation_trigger.run_if(in_state(SimulationState::Simulating)),
                update_generation,
                reset_generation,
                world_clear,
            )
                .run_if(in_state(GameState::Game)),
        );
        app.insert_resource(SpaceKeyTimer::new());
        app.init_state::<SimulationState>();
        app.add_event::<ProgressGenerationEvent>();
        app.add_event::<GenerationResetEvent>();
        app.add_event::<WorldClearEvent>();
    }
}

#[derive(Component)]
struct SideMenuCamera;

pub fn setup_side_menu_camera(mut commands: Commands) {
    commands.spawn((Camera2d, SideMenuCamera, Layer::SideMenu.as_render_layer()));
}

#[derive(Component)]
struct WorldCamera;

pub fn setup_world_camera(mut commands: Commands) {
    commands.spawn((Camera2d, WorldCamera, Layer::World.as_render_layer()));
}

fn setup_resource(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.insert_resource(World::new(
        game_assets.world_width,
        game_assets.world_height,
    ));
    commands.insert_resource(SimulationTimer::new(game_assets.tick_interval));
}
