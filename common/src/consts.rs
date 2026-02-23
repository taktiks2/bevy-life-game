//! ゲーム全体で使用する定数定義

use bevy::color::Color;

// ウィンドウサイズ
/// ウィンドウの初期幅（ピクセル）
pub const WINDOW_WIDTH: f32 = 1280.;
/// ウィンドウの初期高さ（ピクセル）
pub const WINDOW_HEIGHT: f32 = 720.;
/// ウィンドウの最小幅（ピクセル）
pub const MIN_WINDOW_WIDTH: f32 = 600.0;
/// ウィンドウの最小高さ（ピクセル）
pub const MIN_WINDOW_HEIGHT: f32 = 480.0;

// ビューポート
/// ボトムパネルの固定高さ（物理ピクセル）
pub(crate) const PANEL_HEIGHT: u32 = 80;

/// ウィンドウの物理サイズから計算されたビューポートサイズ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewportSizes {
    /// ビューポート全幅
    pub viewport_width: u32,
    /// ワールドカメラの高さ（物理ピクセル）
    pub main_height: u32,
    /// ボトムパネルの高さ（物理ピクセル）
    pub panel_height: u32,
}

/// ウィンドウの物理サイズからビューポートサイズを計算する
///
/// ボトムパネルの高さは `PANEL_HEIGHT` で固定し、残りをワールドカメラに割り当てる。
pub fn calc_viewport_sizes(physical_width: u32, physical_height: u32) -> ViewportSizes {
    let panel_height = PANEL_HEIGHT.min(physical_height);
    let main_height = physical_height - panel_height;
    ViewportSizes {
        viewport_width: physical_width,
        main_height,
        panel_height,
    }
}

// チャンク設定
/// 1チャンクの1辺のセル数
pub const CHUNK_SIZE: i32 = 64;
/// 1セルのワールド空間サイズ
pub const CELL_WORLD_SIZE: f32 = 1.0;
/// 1チャンクのワールド空間サイズ
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_SIZE as f32 * CELL_WORLD_SIZE;

// シミュレーション速度
/// デフォルトのティック間隔（秒）
pub const DEFAULT_TICK_INTERVAL: f32 = 1.0;
/// ティック間隔の最小値（秒）
pub const MIN_TICK_INTERVAL: f32 = 0.1;
/// ティック間隔の最大値（秒）
pub const MAX_TICK_INTERVAL: f32 = 2.0;

// スペースキー長押し判定
/// スペースキーを「長押し」と判定するまでの時間（秒）
pub const SPACE_KEY_HOLD_DURATION: f32 = 0.5;

// カメラ設定
/// カメラの初期ズームスケール
pub const INITIAL_CAMERA_SCALE: f32 = 0.1;
/// カメラのズーム最小値（最も拡大）
pub const MIN_CAMERA_SCALE: f32 = 0.05;
/// カメラのズーム最大値（最も縮小）
pub const MAX_CAMERA_SCALE: f32 = 0.25;
/// ズーム変更時のステップ幅
pub const CAMERA_SCALE_STEP: f32 = 0.01;
/// WASD操作によるカメラ移動速度
pub const CAMERA_PAN_SPEED: f32 = 10.0;
/// ドラッグ判定の移動ピクセル閾値（スクリーン座標）
pub const DRAG_THRESHOLD: f32 = 5.0;
/// マウスホイールズーム感度
pub const MOUSE_WHEEL_ZOOM_SENSITIVITY: f32 = 0.1;

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
/// Genカウンターの最小幅（ピクセル）
pub const GEN_COUNTER_MIN_WIDTH: f32 = 120.0;
/// アクションボタンの最小幅（ピクセル）
pub const ACTION_BUTTON_MIN_WIDTH: f32 = 90.0;
/// ボタンの角丸半径
pub const BORDER_RADIUS: f32 = 8.0;
/// タイトル画面のボタン幅
pub const TITLE_BUTTON_WIDTH: f32 = 200.0;
/// タイトル画面のボタン高さ
pub const TITLE_BUTTON_HEIGHT: f32 = 60.0;
/// タイトル/メニュー画面の上下パディング
pub const TITLE_PADDING: f32 = 80.0;

// パターンボタンUI
/// パターンボタンの幅（ピクセル）
pub const PATTERN_BUTTON_WIDTH: f32 = 140.0;
/// パターンボタンの高さ（ピクセル）
pub const PATTERN_BUTTON_HEIGHT: f32 = 40.0;

// スライダーUI
/// スライダートラックの幅（ピクセル）
pub const SLIDER_TRACK_WIDTH: f32 = 100.0;
/// スライダートラックの高さ（ピクセル）
pub const SLIDER_TRACK_HEIGHT: f32 = 8.0;
/// スライダーサムのサイズ（ピクセル）
pub const SLIDER_THUMB_SIZE: f32 = 20.0;

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

/// セル1個を表現するテクスチャピクセル数（幅・高さ）
pub const CELL_PIXELS: u32 = 8;
/// グリッドラインのRGB色（控えめな暗灰色）
pub const GRID_LINE_RGB: (u8, u8, u8) = (25, 26, 32);
/// グリッド線のスクリーンピクセル幅（ズームレベルに依存しない一定幅）
pub const GRID_LINE_SCREEN_WIDTH: f32 = 0.1;

/// 1チャンクのテクスチャピクセル数（1辺）
/// グリッド線はシェーダーで描画するため、テクスチャにはセルデータのみ
pub const CHUNK_TEX_SIZE: u32 = CHUNK_SIZE as u32 * CELL_PIXELS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_viewport_sizes_default_window() {
        let sizes = calc_viewport_sizes(1280, 720);
        assert_eq!(sizes.viewport_width, 1280);
        assert_eq!(sizes.main_height, 640);
        assert_eq!(sizes.panel_height, PANEL_HEIGHT);
    }

    #[test]
    fn calc_viewport_sizes_panel_height_is_fixed() {
        // ウィンドウの高さが変わってもパネル高さは固定
        for height in [480, 600, 800, 1080, 1440] {
            let sizes = calc_viewport_sizes(1000, height);
            assert_eq!(
                sizes.panel_height, PANEL_HEIGHT,
                "panel_height should be fixed at {PANEL_HEIGHT} for window height={height}"
            );
        }
    }

    #[test]
    fn calc_viewport_sizes_main_plus_panel_equals_total() {
        for height in [480, 600, 800, 1080, 1440] {
            let sizes = calc_viewport_sizes(1000, height);
            assert_eq!(
                sizes.main_height + sizes.panel_height,
                height,
                "main + panel should equal total for height={height}"
            );
        }
    }

    #[test]
    fn calc_viewport_sizes_preserves_width() {
        for width in [600, 800, 1920, 2560] {
            let sizes = calc_viewport_sizes(width, 800);
            assert_eq!(sizes.viewport_width, width);
        }
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;

    #[test]
    fn chunk_tex_size_value() {
        // 64 * 8 = 512 (グリッド線はシェーダー描画のためテクスチャに含まない)
        assert_eq!(CHUNK_TEX_SIZE, 512);
    }

    #[test]
    fn chunk_world_size_value() {
        assert_eq!(CHUNK_WORLD_SIZE, 64.0);
    }

    #[test]
    fn cell_world_size_value() {
        assert_eq!(CELL_WORLD_SIZE, 1.0);
    }
}
