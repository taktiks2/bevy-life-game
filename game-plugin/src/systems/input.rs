use bevy::prelude::*;

use common::states::GameState;

use crate::events::ProgressGenerationEvent;
use crate::resources::timer::SpaceKeyTimer;

pub fn game_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut space_key_timer: ResMut<SpaceKeyTimer>,
    mut progress_generation_event_writer: EventWriter<ProgressGenerationEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        progress_generation_event_writer.send(ProgressGenerationEvent);
    }
    // NOTE: 押し続けている間発火
    if keys.pressed(KeyCode::Space) {
        space_key_timer.0.tick(time.delta()); // NOTE: space_key_timerに経過した時間を加算する
        if space_key_timer.0.finished() {
            progress_generation_event_writer.send(ProgressGenerationEvent);
        }
    }
    // NOTE: キーを離したときにタイマーをリセット
    if keys.just_released(KeyCode::Space) {
        space_key_timer.0.reset();
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        state.set(GameState::Menu);
    }
}
