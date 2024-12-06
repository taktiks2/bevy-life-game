use bevy::prelude::*;
use common::{resources::GameAssets, states::GameState, systems::despawn_screen};

mod components;
mod events;
mod resources;
mod states;
mod systems;

use components::screen::OnGameScreen;
use events::*;
use resources::{timer::SimulationTimer, world::World};
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
            (setup_resource, spawn_screen).chain(),
        );
        app.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
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
            )
                .run_if(in_state(GameState::Game)),
        );
        app.init_state::<SimulationState>();
        app.add_event::<ProgressGenerationEvent>();
    }
}

fn setup_resource(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.insert_resource(World::new(
        game_assets.world_width,
        game_assets.world_height,
    ));
    commands.insert_resource(SimulationTimer::new(game_assets.tick_interval));
}
