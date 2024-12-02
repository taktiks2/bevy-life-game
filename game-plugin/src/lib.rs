use bevy::prelude::*;
use common::{
    consts::{WORLD_HEIGHT, WORLD_WIDTH},
    states::GameState,
    systems::despawn_screen,
};

mod components;
mod events;
mod resources;
mod systems;

use components::screen::OnGameScreen;
use events::*;
use resources::{timer::SpaceKeyTimer, world::World};
use systems::{cell_operations::*, input::game_input_keyboard_handling, screen::spawn_screen};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_screen);
        app.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
        app.insert_resource(World::new(WORLD_WIDTH, WORLD_HEIGHT));
        app.insert_resource(SpaceKeyTimer::new(0.2));
        app.add_systems(
            Update,
            (
                switch_cell_state,
                update_cells,
                game_input_keyboard_handling,
                progress_generation,
            )
                .run_if(in_state(GameState::Game)),
        );
        app.add_event::<ProgressGenerationEvent>();
    }
}
