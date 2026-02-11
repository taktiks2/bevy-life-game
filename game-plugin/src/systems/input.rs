use bevy::prelude::*;

use common::states::GameState;

use crate::events::ProgressGenerationEvent;
use crate::resources::timer::SpaceKeyTimer;
use crate::states::SimulationState;
use crate::WorldCamera;

pub fn game_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut progress_generation_event_writer: MessageWriter<ProgressGenerationEvent>,
    time: Res<Time>,
    mut space_key_timer: ResMut<SpaceKeyTimer>,
) {
    if keys.just_pressed(KeyCode::Space) {
        progress_generation_event_writer.write(ProgressGenerationEvent);
    }
    if keys.pressed(KeyCode::Space)
        && space_key_timer.0.tick(time.delta()).is_finished()
        && *simulation_state.get() == SimulationState::Paused
    {
        // NOTE: スペースキーを長押しして、PausedのときにSimulating開始
        simulation_next_state.set(SimulationState::Simulating);
    }
    if keys.just_released(KeyCode::Space) {
        // NOTE: スペースキーを離したら必ずタイマーをリセット
        space_key_timer.0.reset();
        if *simulation_state.get() == SimulationState::Simulating {
            // NOTE: Simulating中だったら、Pausedにする
            simulation_next_state.set(SimulationState::Paused);
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        game_next_state.set(GameState::Menu);
    }
}

pub fn game_input_zoom_handling(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<WorldCamera>>,
) {
    let Ok((mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::KeyW) {
        transform.translation.y += 10.;
    }
    if keys.just_pressed(KeyCode::KeyS) {
        transform.translation.y -= 10.;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        transform.translation.x -= 10.;
    }
    if keys.just_pressed(KeyCode::KeyD) {
        transform.translation.x += 10.;
    }
    if let Projection::Orthographic(ref mut ortho) = *projection {
        if keys.just_pressed(KeyCode::KeyQ) {
            ortho.scale = (ortho.scale + 0.1).min(1.0);
        }
        if keys.just_pressed(KeyCode::KeyE) {
            ortho.scale = (ortho.scale - 0.1).max(0.1);
        }
    }
}
