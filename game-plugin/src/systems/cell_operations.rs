use bevy::prelude::*;

use crate::components::{coordinate::Coordinate, screen::GenerationText};
use crate::events::{GenerationResetEvent, ProgressGenerationEvent, WorldClearEvent};
use crate::resources::{
    cell_materials::CellMaterials,
    timer::SimulationTimer,
    world::{Cell, World},
};

pub fn update_cells(
    world: Res<World>,
    cell_materials: Res<CellMaterials>,
    mut query: Query<(&Coordinate, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    if !world.is_changed() {
        return;
    }
    for (coordinate, mut material) in query.iter_mut() {
        let handle = match world.cells[coordinate.y as usize][coordinate.x as usize] {
            Cell::Alive => cell_materials.alive.clone(),
            Cell::Dead => cell_materials.dead.clone(),
        };
        *material = MeshMaterial2d(handle);
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
