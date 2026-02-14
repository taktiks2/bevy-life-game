//! ゲーム全体で使用する定数定義

use bevy::color::Color;

// ウィンドウサイズ
/// ウィンドウの幅（ピクセル）
pub const WINDOW_WIDTH: f32 = 1000.;
/// ウィンドウの高さ（ピクセル）
pub const WINDOW_HEIGHT: f32 = 800.;
/// ビューポート全幅
pub const VIEWPORT_WIDTH: u32 = WINDOW_WIDTH as u32;
/// ボトムパネルのビューポート高さ（ウィンドウ高さの20%）
pub const PANEL_HEIGHT: u32 = (WINDOW_HEIGHT * 0.1) as u32;
/// メインビュー（ワールド表示）のビューポート高さ（ウィンドウ高さの80%）
pub const MAIN_HEIGHT: u32 = (WINDOW_HEIGHT * 0.9) as u32;

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
pub const FONT_SIZE_LARGE: f32 = 24.0;
/// 中程度のフォントサイズ（世代カウンター等）
pub const FONT_SIZE_MEDIUM: f32 = 20.0;
/// 小さいフォントサイズ（ステッパーラベル）
pub const FONT_SIZE_SMALL: f32 = 16.0;
/// アクションボタンの高さ
pub const ACTION_BUTTON_HEIGHT: f32 = 44.0;
/// ボタンの角丸半径
pub const BORDER_RADIUS: f32 = 8.0;
/// タイトル画面のボタン幅
pub const TITLE_BUTTON_WIDTH: f32 = 200.0;
/// タイトル画面のボタン高さ
pub const TITLE_BUTTON_HEIGHT: f32 = 60.0;
/// タイトル/メニュー画面の上下パディング
pub const TITLE_PADDING: f32 = 200.0;

// テーマカラー: 背景系（暗い順）
/// グリッド背景色
pub const BG_DARKEST: Color = Color::srgb(0.06, 0.06, 0.08);
/// メイン背景色（タイトル/メニュー画面）
pub const BG_DARK: Color = Color::srgb(0.09, 0.09, 0.12);
/// サーフェス色（サイドバー等）
pub const BG_SURFACE: Color = Color::srgb(0.12, 0.13, 0.16);
/// ボタン通常色
pub const BG_BUTTON: Color = Color::srgb(0.16, 0.17, 0.21);
/// ボタンホバー色（緑がかり）
pub const BG_BUTTON_HOVER: Color = Color::srgb(0.20, 0.24, 0.22);

// テーマカラー: アクセント（グリーン系）
/// メインアクセント色（ネオングリーン）
pub const ACCENT_GREEN: Color = Color::srgb(0.0, 0.85, 0.45);
/// 控えめアクセント色
pub const ACCENT_GREEN_DIM: Color = Color::srgb(0.0, 0.55, 0.30);

// テーマカラー: テキスト
/// 主要テキスト色
pub const TEXT_PRIMARY: Color = Color::srgb(0.92, 0.93, 0.95);
/// 控えめテキスト色
pub const TEXT_MUTED: Color = Color::srgb(0.50, 0.52, 0.58);

// テーマカラー: ボーダー
/// 控えめボーダー色
pub const BORDER_SUBTLE: Color = Color::srgb(0.20, 0.21, 0.26);
/// ボタンボーダー幅
pub const BUTTON_BORDER_WIDTH: f32 = 1.5;
/// ボタンホバー時のボーダー色（ネオングリーン）
pub const BUTTON_BORDER_HOVER: Color = Color::srgb(0.0, 0.85, 0.45);

// テーマカラー: セル描画用RGB値
/// 生存セルのRGB値（ネオングリーン）
pub const CELL_ALIVE_RGB: (u8, u8, u8) = (0, 217, 115);
/// 死亡セルのRGB値（ほぼ黒）
pub const CELL_DEAD_RGB: (u8, u8, u8) = (15, 15, 20);

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

/// グリッドのワールド空間表示サイズ（正方形）
pub const GRID_DISPLAY_SIZE: f32 = 800.0;

/// 指定ワールドサイズに対する1セルのピクセルサイズ (幅, 高さ) を返す
pub fn cell_size(world_width: u16, world_height: u16) -> (f32, f32) {
    (
        GRID_DISPLAY_SIZE / world_width as f32,
        GRID_DISPLAY_SIZE / world_height as f32,
    )
}
