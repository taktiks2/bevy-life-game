//! 複数のプラグインで再利用される汎用システム

use bevy::prelude::*;

/// 指定コンポーネントを持つ全エンティティを削除する汎用システム
///
/// 画面遷移時のクリーンアップに使用する。
/// 例: `OnExit(GameState::Title)` で `despawn_entity::<OnTitleScreen>` を登録する。
pub fn despawn_entity<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

/// 2Dカメラを生成する汎用システム
///
/// マーカーコンポーネントを付与したカメラを生成する。
/// 各画面のカメラ初期化に使用する。
pub fn setup_camera<T: Component>(mut commands: Commands, camera_type: T) {
    commands.spawn((Camera2d, camera_type));
}
