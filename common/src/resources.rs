//! アプリケーション全体で共有されるリソース

use bevy::prelude::{AssetServer, AudioSource, Font, FromWorld, Handle, Resource, World};

use crate::consts::DEFAULT_TICK_INTERVAL;

/// ゲーム全体で使用するアセットと設定を保持するリソース
///
/// フォント・効果音のハンドルに加え、シミュレーション速度など
/// 各プラグインから参照される設定値を含む。
#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    /// 通常テキスト用フォント
    pub font: Handle<Font>,
    /// 見出し・ボタン用太字フォント
    pub font_bold: Handle<Font>,
    /// ホバー時の効果音
    pub audio_hover: Handle<AudioSource>,
    /// シミュレーションのティック間隔（秒）
    pub tick_interval: f32,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
            font_bold: asset_server.load("fonts/PixelMplus12-Bold.ttf"),
            audio_hover: asset_server.load("audios/appear-online.ogg"),
            tick_interval: DEFAULT_TICK_INTERVAL,
        }
    }
}
