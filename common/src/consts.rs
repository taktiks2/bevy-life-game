//! ゲーム全体で使用する定数定義

// ウィンドウサイズ
/// ウィンドウの幅（ピクセル）
pub const WINDOW_WIDTH: f32 = 1000.;
/// ウィンドウの高さ（ピクセル）
pub const WINDOW_HEIGHT: f32 = 800.;
/// サイドメニューの物理幅（ウィンドウ幅の20%）
pub const SUB_PHYSICAL_WIDTH: u32 = (WINDOW_WIDTH * 0.2) as u32;
/// メインビュー（ワールド表示）の物理幅（ウィンドウ幅の80%）
pub const MAIN_PHYSICAL_WIDTH: u32 = (WINDOW_WIDTH * 0.8) as u32;
/// レンダリング用の物理高さ（Retina対応で2倍）
pub const PHYSICAL_HEIGHT: u32 = (WINDOW_HEIGHT as u32) * 2;

// ワールドサイズ（セル数）
/// ワールドの幅（セル数）
pub const WORLD_WIDTH: u16 = 100;
/// ワールドの高さ（セル数）
pub const WORLD_HEIGHT: u16 = 100;

// シミュレーション速度
/// デフォルトのティック間隔（秒）
pub const DEFAULT_TICK_INTERVAL: f32 = 0.2;
/// ティック間隔の最小値（秒）
pub const MIN_TICK_INTERVAL: f32 = 0.1;
/// ティック間隔の最大値（秒）
pub const MAX_TICK_INTERVAL: f32 = 5.0;
/// 速度変更時のステップ幅（秒）
pub const TICK_INTERVAL_STEP: f32 = 0.1;

// スペースキー長押し判定
/// スペースキーを「長押し」と判定するまでの時間（秒）
pub const SPACE_KEY_HOLD_DURATION: f32 = 0.5;

// カメラ設定
/// カメラの初期ズームスケール
pub const INITIAL_CAMERA_SCALE: f32 = 0.5;
/// カメラのズーム最小値（最も拡大）
pub const MIN_CAMERA_SCALE: f32 = 0.1;
/// カメラのズーム最大値（最も縮小）
pub const MAX_CAMERA_SCALE: f32 = 1.0;
/// ズーム変更時のステップ幅
pub const CAMERA_SCALE_STEP: f32 = 0.1;
/// WASD操作によるカメラ移動速度
pub const CAMERA_PAN_SPEED: f32 = 10.0;

/// ムーア近傍の8方向の相対座標
///
/// あるセルの周囲8マスを走査するために使用する。
pub const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
