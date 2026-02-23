use bevy::prelude::*;
use common::resources::{AudioMuted, GameAssets};

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
    audio_muted: Res<AudioMuted>,
) {
    // クールダウンタイマーを経過時間分進める
    cooldown.0.tick(time.delta());

    // ミュート中はイベントを読み捨てる
    if audio_muted.0 {
        for _ in events.read() {}
        return;
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::time::TimeUpdateStrategy;
    use common::resources::AudioMuted;
    use std::time::Duration;

    /// テスト用にPlayAudioEventを送信するシステム
    fn send_play_audio_event(mut writer: MessageWriter<PlayAudioEvent>) {
        writer.write(PlayAudioEvent);
    }

    /// テスト用のアプリを構築するヘルパー
    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(bevy::audio::AudioPlugin::default());

        // 固定時間を設定（100ms進める）
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));

        // リソースを初期化
        let audio_hover = app
            .world_mut()
            .resource::<AssetServer>()
            .load::<AudioSource>("audios/appear-online.ogg");
        app.insert_resource(GameAssets {
            font: Handle::default(),
            font_bold: Handle::default(),
            audio_hover,
            tick_interval: 0.5,
        });

        // クールダウンタイマー（50ms、初期状態は完了済み）
        let mut timer = Timer::from_seconds(0.05, TimerMode::Once);
        timer.tick(Duration::from_millis(50));
        app.insert_resource(AudioCooldown(timer));

        // イベント登録
        app.add_message::<PlayAudioEvent>();

        // イベント送信 → play_audios の順で実行
        app.add_systems(Update, send_play_audio_event.before(play_audios));
        app.add_systems(Update, play_audios);

        app
    }

    /// スポーンされたAudioPlayerの数を返すヘルパー
    fn count_audio_players(app: &mut App) -> usize {
        app.world_mut()
            .query::<&AudioPlayer<AudioSource>>()
            .iter(app.world())
            .count()
    }

    #[test]
    fn mute_disables_audio_playback() {
        let mut app = setup_test_app();
        app.insert_resource(AudioMuted(true));

        // 1回目: send_play_audio_event がメッセージ送信
        // 2回目: play_audios がメッセージ受信（メッセージは次フレームで配信）
        app.update();
        app.update();

        assert_eq!(
            count_audio_players(&mut app),
            0,
            "ミュート時はAudioPlayerがスポーンされないべき"
        );
    }

    #[test]
    fn unmute_enables_audio_playback() {
        let mut app = setup_test_app();
        app.insert_resource(AudioMuted(false));

        // 1回目: send_play_audio_event がメッセージ送信
        // 2回目: play_audios がメッセージ受信（メッセージは次フレームで配信）
        app.update();
        app.update();

        assert_eq!(
            count_audio_players(&mut app),
            1,
            "ミュート解除時はAudioPlayerがスポーンされるべき"
        );
    }

    #[test]
    fn default_is_not_muted() {
        let muted = AudioMuted::default();
        assert!(!muted.0, "デフォルトではミュートされていないべき");
    }
}
