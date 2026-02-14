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

// UIサイズ
/// タイトルのフォントサイズ
pub const FONT_SIZE_TITLE: f32 = 60.0;
/// 大きいフォントサイズ（ボタンラベル）
pub const FONT_SIZE_LARGE: f32 = 40.0;
/// 中程度のフォントサイズ（世代カウンター等）
pub const FONT_SIZE_MEDIUM: f32 = 30.0;
/// 小さいフォントサイズ（ステッパーラベル）
pub const FONT_SIZE_SMALL: f32 = 20.0;
/// アクションボタンの高さ
pub const ACTION_BUTTON_HEIGHT: f32 = 60.0;
/// ボタンの角丸半径
pub const BORDER_RADIUS: f32 = 5.0;
/// タイトル画面のボタン幅
pub const TITLE_BUTTON_WIDTH: f32 = 200.0;
/// タイトル画面のボタン高さ
pub const TITLE_BUTTON_HEIGHT: f32 = 60.0;
/// タイトル/メニュー画面の上下パディング
pub const TITLE_PADDING: f32 = 200.0;

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

/// 指定ワールドサイズに対する1セルのピクセルサイズ (幅, 高さ) を返す
pub fn cell_size(world_width: u16, world_height: u16) -> (f32, f32) {
    (
        MAIN_PHYSICAL_WIDTH as f32 / world_width as f32,
        WINDOW_HEIGHT / world_height as f32,
    )
}
