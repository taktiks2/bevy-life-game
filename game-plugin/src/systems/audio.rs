use bevy::prelude::*;
use common::resources::GameAssets;

use crate::events::PlayAudioEvent;
use crate::resources::interaction::AudioCooldown;

/// PlayAudioEvent を受け取り、クールダウン制御付きで効果音を再生する
///
/// - 50ms以内の連続イベントは無視し、音が重なって大音量になるのを防止
/// - 再生完了後はエンティティを自動削除（PlaybackSettings::DESPAWN）
pub fn play_audios(
    game_assets: Res<GameAssets>,
    mut events: MessageReader<PlayAudioEvent>,
    mut commands: Commands,
    mut cooldown: ResMut<AudioCooldown>,
    time: Res<Time>,
) {
    // クールダウンタイマーを経過時間分進める
    cooldown.0.tick(time.delta());

    // 溜まっている PlayAudioEvent を全て読み出す
    for _ in events.read() {
        // タイマーが完了済み（50ms経過）なら音を鳴らす
        if cooldown.0.is_finished() {
            // AudioPlayer エンティティをスポーンして再生
            commands.spawn((
                AudioPlayer::new(game_assets.audio_hover.clone()),
                PlaybackSettings::DESPAWN,
            ));
            // タイマーをリセット → 次の50msはイベントが来ても再生しない
            cooldown.0.reset();
        }
        // クールダウン中のイベントは読み捨てられる
    }
}
