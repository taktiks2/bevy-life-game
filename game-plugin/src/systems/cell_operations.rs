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

// TODO: テキストをアップデートする関数を作成する
pub fn update_generation(world: Res<World>, mut query: Query<&mut Text, With<GenerationText>>) {
    for mut generation_text in query.iter_mut() {
        generation_text.0 = format!("Gen: {}", world.generation_count);
    }
}

pub fn progress_generation(
    mut world: ResMut<World>,
    mut progress_generation_event_reader: EventReader<ProgressGenerationEvent>,
) {
    for _ in progress_generation_event_reader.read() {
        world.generation_count += 1;
        println!("Generation: {}", world.generation_count);
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
