use bevy::prelude::*;

use crate::components::coordinate::Coordinate;
use crate::events::{GenerationResetEvent, ProgressGenerationEvent, WorldClearEvent};
use crate::resources::{timer::SimulationTimer, world::World};
use crate::states::SimulationState;
use crate::WorldCamera;

pub fn handle_start(
    _click: Trigger<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Paused {
        simulation_next_state.set(SimulationState::Simulating);
    }
}

pub fn handle_stop(
    _click: Trigger<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Simulating {
        simulation_next_state.set(SimulationState::Paused);
    }
}

pub fn handle_next(
    _click: Trigger<Pointer<Click>>,
    mut progress_generation_event_writer: EventWriter<ProgressGenerationEvent>,
) {
    progress_generation_event_writer.send(ProgressGenerationEvent);
}

pub fn handle_reset(
    _click: Trigger<Pointer<Click>>,
    mut generation_reset_event_writer: EventWriter<GenerationResetEvent>,
) {
    generation_reset_event_writer.send(GenerationResetEvent);
}

pub fn handle_clear(
    _click: Trigger<Pointer<Click>>,
    mut world_clear_event_writer: EventWriter<WorldClearEvent>,
) {
    world_clear_event_writer.send(WorldClearEvent);
}

pub fn handle_speed_down(
    _click: Trigger<Pointer<Click>>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    let current_duration = simulation_timer.0.duration().as_secs_f32();
    let new_duration = (current_duration + 0.1).min(5.0); // 最大5秒まで増加
    simulation_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(new_duration));
}

pub fn handle_speed_up(
    _click: Trigger<Pointer<Click>>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    let current_duration = simulation_timer.0.duration().as_secs_f32();
    let new_duration = (current_duration - 0.1).max(0.1); // 最小0.1秒まで減少
    simulation_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(new_duration));
}

pub fn handle_zoom_down(
    _click: Trigger<Pointer<Click>>,
    mut query_camera: Query<&mut OrthographicProjection, With<WorldCamera>>,
) {
    for mut camera in query_camera.iter_mut() {
        camera.scale = (camera.scale + 0.1).min(1.0);
    }
}

pub fn handle_zoom_up(
    _click: Trigger<Pointer<Click>>,
    mut query_camera: Query<&mut OrthographicProjection, With<WorldCamera>>,
) {
    for mut camera in query_camera.iter_mut() {
        camera.scale = (camera.scale - 0.1).max(0.1);
    }
}

pub fn switch_cell_state(
    click: Trigger<Pointer<Click>>,
    query: Query<&Coordinate, With<Mesh2d>>,
    mut world: ResMut<World>,
) {
    if let Ok(coordinate) = query.get(click.entity()) {
        world.cells[coordinate.y as usize][coordinate.x as usize] =
            world.cells[coordinate.y as usize][coordinate.x as usize].switch_state();
        world.generation_count = 0;
        world.prev_cells = world.cells.clone();
    }
}
