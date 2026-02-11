use bevy::{color::palettes::css::NAVY, prelude::*};
use common::consts::{
    MAIN_PHYSICAL_WIDTH, MAX_CAMERA_SCALE, MAX_TICK_INTERVAL, MIN_CAMERA_SCALE,
    MIN_TICK_INTERVAL, WINDOW_HEIGHT, CAMERA_SCALE_STEP, TICK_INTERVAL_STEP,
};

use crate::components::screen::CellHighlight;
use crate::events::{
    GenerationResetEvent, PlayAudioEvent, ProgressGenerationEvent, WorldClearEvent,
};
use crate::resources::{timer::SimulationTimer, world::World};
use crate::states::SimulationState;
use crate::WorldCamera;

#[derive(Resource, Default, PartialEq)]
pub struct HoveredCell(pub Option<(u16, u16)>);

pub fn handle_start(
    _click: On<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Paused {
        simulation_next_state.set(SimulationState::Simulating);
    }
}

pub fn handle_stop(
    _click: On<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Simulating {
        simulation_next_state.set(SimulationState::Paused);
    }
}

pub fn handle_next(
    _click: On<Pointer<Click>>,
    mut progress_generation_event_writer: MessageWriter<ProgressGenerationEvent>,
) {
    progress_generation_event_writer.write(ProgressGenerationEvent);
}

pub fn handle_reset(
    _click: On<Pointer<Click>>,
    mut generation_reset_event_writer: MessageWriter<GenerationResetEvent>,
) {
    generation_reset_event_writer.write(GenerationResetEvent);
}

pub fn handle_clear(
    _click: On<Pointer<Click>>,
    mut world_clear_event_writer: MessageWriter<WorldClearEvent>,
) {
    world_clear_event_writer.write(WorldClearEvent);
}

pub fn handle_speed_down(
    _click: On<Pointer<Click>>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    let current_duration = simulation_timer.0.duration().as_secs_f32();
    let new_duration = (current_duration + TICK_INTERVAL_STEP).min(MAX_TICK_INTERVAL);
    simulation_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(new_duration));
}

pub fn handle_speed_up(
    _click: On<Pointer<Click>>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    let current_duration = simulation_timer.0.duration().as_secs_f32();
    let new_duration = (current_duration - TICK_INTERVAL_STEP).max(MIN_TICK_INTERVAL);
    simulation_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(new_duration));
}

pub fn handle_zoom_down(
    _click: On<Pointer<Click>>,
    mut query_camera: Query<&mut Projection, With<WorldCamera>>,
) {
    for mut projection in query_camera.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = (ortho.scale + CAMERA_SCALE_STEP).min(MAX_CAMERA_SCALE);
        }
    }
}

pub fn handle_zoom_up(
    _click: On<Pointer<Click>>,
    mut query_camera: Query<&mut Projection, With<WorldCamera>>,
) {
    for mut projection in query_camera.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = (ortho.scale - CAMERA_SCALE_STEP).max(MIN_CAMERA_SCALE);
        }
    }
}

pub fn handle_over(
    over: On<Pointer<Over>>,
    mut query: Query<&mut BackgroundColor>,
    mut events: MessageWriter<PlayAudioEvent>,
) {
    if let Ok(mut background_color) = query.get_mut(over.entity) {
        background_color.0 = NAVY.into();
        events.write(PlayAudioEvent);
    }
}

pub fn handle_out(out: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>) {
    if let Ok(mut background_color) = query.get_mut(out.entity) {
        background_color.0 = Color::BLACK;
    }
}

pub fn world_to_screen_pos(grid_x: u16, grid_y: u16, world_width: u16, world_height: u16) -> Vec2 {
    let cell_w = MAIN_PHYSICAL_WIDTH as f32 / world_width as f32;
    let cell_h = WINDOW_HEIGHT / world_height as f32;
    Vec2::new(
        grid_x as f32 * cell_w - MAIN_PHYSICAL_WIDTH as f32 / 2.0 + cell_w / 2.0,
        -(grid_y as f32 * cell_h - WINDOW_HEIGHT / 2.0 + cell_h / 2.0),
    )
}

pub fn screen_to_grid_coords(
    world_pos: Vec2,
    world_width: u16,
    world_height: u16,
) -> Option<(u16, u16)> {
    let half_w = MAIN_PHYSICAL_WIDTH as f32 / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;

    let local_x = world_pos.x + half_w;
    let local_y = -world_pos.y + half_h;

    if local_x < 0.0 || local_y < 0.0 {
        return None;
    }

    let cell_w = MAIN_PHYSICAL_WIDTH as f32 / world_width as f32;
    let cell_h = WINDOW_HEIGHT / world_height as f32;

    let grid_x = (local_x / cell_w) as u16;
    let grid_y = (local_y / cell_h) as u16;

    if grid_x >= world_width || grid_y >= world_height {
        return None;
    }

    Some((grid_x, grid_y))
}

pub fn handle_grid_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    mut world: ResMut<World>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, transform)) = camera_query.single() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(transform, cursor_pos) else {
        return;
    };
    if let Some((gx, gy)) = screen_to_grid_coords(world_pos, world.width, world.height) {
        world.toggle_cell(gx, gy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MAIN_PHYSICAL_WIDTH = 800, WINDOW_HEIGHT = 800, 100x100 grid
    // cell size = 8x8 pixels

    #[test]
    fn center_of_grid_returns_correct_coords() {
        // World pos (0, 0) = center of grid = cell (50, 50) approximately
        let result = screen_to_grid_coords(Vec2::new(0.0, 0.0), 100, 100);
        assert_eq!(result, Some((50, 50)));
    }

    #[test]
    fn top_left_corner_returns_0_0() {
        // Top-left: world pos (-400, 400)
        let result = screen_to_grid_coords(Vec2::new(-400.0, 400.0), 100, 100);
        assert_eq!(result, Some((0, 0)));
    }

    #[test]
    fn bottom_right_returns_99_99() {
        // Bottom-right: world pos (399, -399)
        let result = screen_to_grid_coords(Vec2::new(399.0, -399.0), 100, 100);
        assert_eq!(result, Some((99, 99)));
    }

    #[test]
    fn outside_grid_left_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(-401.0, 0.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn outside_grid_right_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(401.0, 0.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn outside_grid_top_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(0.0, 401.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn outside_grid_bottom_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(0.0, -401.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn world_to_screen_and_back_roundtrips() {
        for gx in [0u16, 25, 50, 75, 99] {
            for gy in [0u16, 25, 50, 75, 99] {
                let screen_pos = world_to_screen_pos(gx, gy, 100, 100);
                let result = screen_to_grid_coords(screen_pos, 100, 100);
                assert_eq!(result, Some((gx, gy)), "roundtrip failed for ({}, {})", gx, gy);
            }
        }
    }
}

pub fn update_cell_highlight(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    world: Res<World>,
    mut highlight_query: Query<(&mut Transform, &mut Visibility), With<CellHighlight>>,
    mut hovered: ResMut<HoveredCell>,
    mut events: MessageWriter<PlayAudioEvent>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_query.single() else {
        return;
    };

    let grid_coords = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(cam_transform, cursor_pos).ok())
        .and_then(|world_pos| screen_to_grid_coords(world_pos, world.width, world.height));

    let Ok((mut transform, mut vis)) = highlight_query.single_mut() else {
        return;
    };

    match grid_coords {
        Some((gx, gy)) => {
            *vis = Visibility::Inherited;
            let pos = world_to_screen_pos(gx, gy, world.width, world.height);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
            transform.translation.z = 1.0;

            let new_hover = Some((gx, gy));
            if hovered.0 != new_hover {
                hovered.0 = new_hover;
                events.write(PlayAudioEvent);
            }
        }
        None => {
            *vis = Visibility::Hidden;
            hovered.0 = None;
        }
    }
}
