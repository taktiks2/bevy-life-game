use bevy::prelude::*;

use crate::components::{coordinate::Coordinate, screen::GenerationText};
use crate::events::ProgressGenerationEvent;
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
                world.cells[coordinate.y as usize][coordinate.x as usize].switch_state()
        }
    }
}

pub fn update_cells(world: Res<World>, mut query: Query<(&Coordinate, &mut BackgroundColor)>) {
    for (coordinate, mut background_color) in query.iter_mut() {
        let cell = &world.cells[coordinate.y as usize][coordinate.x as usize];
        *background_color = BackgroundColor(cell.get_color());
    }
}

pub fn update_generation(world: Res<World>, mut query: Query<&mut TextSpan, With<GenerationText>>) {
    query.single_mut().0 = world.generation_count.to_string();
}

pub fn progress_generation(
    mut world: ResMut<World>,
    mut progress_generation_event_reader: EventReader<ProgressGenerationEvent>,
) {
    for _ in progress_generation_event_reader.read() {
        world.generation_count += 1;
        world.progress_generation()
    }
}

pub fn progress_generation_trigger(
    time: Res<Time>,
    mut simulation_timer: ResMut<SimulationTimer>,
    mut progress_generation_event_writer: EventWriter<ProgressGenerationEvent>,
) {
    if simulation_timer.0.tick(time.delta()).finished() {
        progress_generation_event_writer.send(ProgressGenerationEvent);
    }
}
