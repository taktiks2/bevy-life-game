//! ゲーム内のシステム間通信に使用するメッセージイベント

use bevy::prelude::Message;

/// 世代を1つ進めるイベント
#[derive(Message)]
pub struct ProgressGenerationEvent;

/// 世代を初期状態にリセットするイベント
#[derive(Message)]
pub struct GenerationResetEvent;

/// ワールドの全セルをクリアするイベント
#[derive(Message)]
pub struct WorldClearEvent;

/// 効果音を再生するイベント
#[derive(Message)]
pub struct PlayAudioEvent;
