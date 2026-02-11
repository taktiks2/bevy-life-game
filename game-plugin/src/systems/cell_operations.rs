use bevy::prelude::*;

use crate::components::{coordinate::Coordinate, screen::GenerationText};
use crate::events::{GenerationResetEvent, ProgressGenerationEvent, WorldClearEvent};
use crate::resources::{timer::SimulationTimer, world::World};

// NOTE: interaction_queryのwarningを出さないようにするには、以下の型をあてること
// type InteractionQuery<'a, 'b> =
//     Query<'a, 'b, (&'a Interaction, &'b Coordinate), (Changed<Interaction>, With<Button>)>;

#[allow(clippy::type_complexity)]
pub fn switch_cell_state(
    mut interaction_query: Query<(&Interaction, &Coordinate), (Changed<Interaction>, With<Button>)>,
    mut world: ResMut<World>,
) {
    for (interaction, coordinate) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            world.cells[coordinate.y as usize][coordinate.x as usize] =
                world.cells[coordinate.y as usize][coordinate.x as usize].switch_state();
            world.generation_count = 0;
            world.prev_cells = world.cells.clone();
        }
    }
}

pub fn update_cells(
    world: Res<World>,
    mut query: Query<(&Coordinate, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (coordinate, mut material) in query.iter_mut() {
        let cell = &world.cells[coordinate.y as usize][coordinate.x as usize];
        *material = MeshMaterial2d(materials.add(cell.get_color()));
    }
}

pub fn update_generation(world: Res<World>, mut query: Query<&mut TextSpan, With<GenerationText>>) {
    if let Ok(mut span) = query.single_mut() {
        span.0 = world.generation_count.to_string();
    }
}

// NOTE: 一定間隔事に進化させる関数
pub fn progress_generation_trigger(
    time: Res<Time>,
    mut simulation_timer: ResMut<SimulationTimer>,
    mut progress_generation_event_writer: MessageWriter<ProgressGenerationEvent>,
) {
    if simulation_timer.0.tick(time.delta()).is_finished() {
        progress_generation_event_writer.write(ProgressGenerationEvent);
    }
}

pub fn progress_generation(
    mut world: ResMut<World>,
    mut progress_generation_event_reader: MessageReader<ProgressGenerationEvent>,
) {
    for _ in progress_generation_event_reader.read() {
        world.progress_generation()
    }
}

pub fn reset_generation(
    mut world: ResMut<World>,
    mut generation_reset_event_reader: MessageReader<GenerationResetEvent>,
) {
    for _ in generation_reset_event_reader.read() {
        world.reset();
    }
}

pub fn world_clear(
    mut world: ResMut<World>,
    mut world_clear_event_reader: MessageReader<WorldClearEvent>,
) {
    for _ in world_clear_event_reader.read() {
        world.clear();
    }
}
