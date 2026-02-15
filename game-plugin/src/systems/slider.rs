//! スライダーUIの変換関数・ドラッグハンドラ・同期システム

use bevy::prelude::*;
use common::consts::{
    MAX_CAMERA_SCALE, MAX_TICK_INTERVAL, MIN_CAMERA_SCALE, MIN_TICK_INTERVAL, SLIDER_THUMB_SIZE,
    SLIDER_TRACK_WIDTH,
};

use crate::components::camera::WorldCamera;
use crate::components::slider::{SliderKind, SliderThumb, SliderTrack, SliderValueText};
use crate::resources::timer::SimulationTimer;

/// tick_interval 値をスライダー比率 (0.0–1.0) に変換する
///
/// 右側ほど速い（tick_interval が小さい）ため、逆転マッピングを行う。
pub fn speed_to_ratio(tick_interval: f32) -> f32 {
    ((MAX_TICK_INTERVAL - tick_interval) / (MAX_TICK_INTERVAL - MIN_TICK_INTERVAL)).clamp(0.0, 1.0)
}

/// スライダー比率 (0.0–1.0) を tick_interval 値に変換する
pub fn ratio_to_speed(ratio: f32) -> f32 {
    let r = ratio.clamp(0.0, 1.0);
    MAX_TICK_INTERVAL - r * (MAX_TICK_INTERVAL - MIN_TICK_INTERVAL)
}

/// camera_scale 値をスライダー比率 (0.0–1.0) に変換する
///
/// 右側ほどズームイン（camera_scale が小さい）ため、逆転マッピングを行う。
pub fn zoom_to_ratio(camera_scale: f32) -> f32 {
    ((MAX_CAMERA_SCALE - camera_scale) / (MAX_CAMERA_SCALE - MIN_CAMERA_SCALE)).clamp(0.0, 1.0)
}

/// スライダー比率 (0.0–1.0) を camera_scale 値に変換する
pub fn ratio_to_zoom(ratio: f32) -> f32 {
    let r = ratio.clamp(0.0, 1.0);
    MAX_CAMERA_SCALE - r * (MAX_CAMERA_SCALE - MIN_CAMERA_SCALE)
}

/// トラック上でのドラッグハンドラ
///
/// ドラッグのdelta.xをトラック幅で正規化し、スライダーの値を更新する。
pub fn handle_slider_drag(
    drag: On<Pointer<Drag>>,
    query_track: Query<(&SliderKind, &ComputedNode), With<SliderTrack>>,
    mut simulation_timer: ResMut<SimulationTimer>,
    mut camera_query: Query<&mut Projection, With<WorldCamera>>,
) {
    let entity = drag.entity;
    let Ok((kind, computed)) = query_track.get(entity) else {
        return;
    };
    let track_width = computed.size().x;
    if track_width <= 0.0 {
        return;
    }
    let delta_ratio = drag.event.delta.x / track_width;

    match kind {
        SliderKind::Speed => {
            let current = simulation_timer.0.duration().as_secs_f32();
            let current_ratio = speed_to_ratio(current);
            let new_ratio = (current_ratio + delta_ratio).clamp(0.0, 1.0);
            let new_interval = ratio_to_speed(new_ratio);
            simulation_timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(new_interval));
        }
        SliderKind::Zoom => {
            for mut projection in camera_query.iter_mut() {
                if let Projection::Orthographic(ref mut ortho) = *projection {
                    let current_ratio = zoom_to_ratio(ortho.scale);
                    let new_ratio = (current_ratio + delta_ratio).clamp(0.0, 1.0);
                    ortho.scale = ratio_to_zoom(new_ratio);
                }
            }
        }
    }
}

/// トラック上のクリックで値を即座に設定するハンドラ
pub fn handle_slider_click(
    click: On<Pointer<Click>>,
    query_track: Query<(&SliderKind, &ComputedNode, &GlobalTransform), With<SliderTrack>>,
    mut simulation_timer: ResMut<SimulationTimer>,
    mut camera_query: Query<&mut Projection, With<WorldCamera>>,
) {
    let entity = click.entity;
    let Ok((kind, computed, global_transform)) = query_track.get(entity) else {
        return;
    };
    let track_width = computed.size().x;
    if track_width <= 0.0 {
        return;
    }

    // クリック位置からトラック内の相対比率を計算
    let track_left = global_transform.translation().x - track_width / 2.0;
    let click_x = click.pointer_location.position.x;
    let ratio = ((click_x - track_left) / track_width).clamp(0.0, 1.0);

    match kind {
        SliderKind::Speed => {
            let new_interval = ratio_to_speed(ratio);
            simulation_timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(new_interval));
        }
        SliderKind::Zoom => {
            for mut projection in camera_query.iter_mut() {
                if let Projection::Orthographic(ref mut ortho) = *projection {
                    ortho.scale = ratio_to_zoom(ratio);
                }
            }
        }
    }
}

/// スライダーのサム位置と値テキストをゲーム状態に同期するシステム
pub fn sync_slider_thumbs(
    simulation_timer: Res<SimulationTimer>,
    camera_query: Query<&Projection, With<WorldCamera>>,
    mut thumb_query: Query<(&SliderKind, &mut Node), With<SliderThumb>>,
    mut text_query: Query<(&SliderKind, &mut Text), With<SliderValueText>>,
) {
    let speed_ratio = speed_to_ratio(simulation_timer.0.duration().as_secs_f32());
    let zoom_ratio = camera_query
        .iter()
        .find_map(|p| {
            if let Projection::Orthographic(ortho) = p {
                Some(zoom_to_ratio(ortho.scale))
            } else {
                None
            }
        })
        .unwrap_or(0.5);

    for (kind, mut node) in thumb_query.iter_mut() {
        let ratio = match kind {
            SliderKind::Speed => speed_ratio,
            SliderKind::Zoom => zoom_ratio,
        };
        // サムの left = ratio * (track_width - thumb_size)
        node.left = Val::Px(ratio * (SLIDER_TRACK_WIDTH - SLIDER_THUMB_SIZE));
    }

    for (kind, mut text) in text_query.iter_mut() {
        match kind {
            SliderKind::Speed => {
                let interval = simulation_timer.0.duration().as_secs_f32();
                **text = format!("{interval:.1}s");
            }
            SliderKind::Zoom => {
                let scale = camera_query
                    .iter()
                    .find_map(|p| {
                        if let Projection::Orthographic(ortho) = p {
                            Some(ortho.scale)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0.5);
                let zoom_pct = ((1.0 / scale) * 100.0) as u32;
                **text = format!("{zoom_pct}%");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === speed_to_ratio テスト ===

    #[test]
    fn speed_to_ratio_min_interval_returns_1() {
        // 最速（MIN_TICK_INTERVAL）は右端 = ratio 1.0
        assert!((speed_to_ratio(MIN_TICK_INTERVAL) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn speed_to_ratio_max_interval_returns_0() {
        // 最遅（MAX_TICK_INTERVAL）は左端 = ratio 0.0
        assert!((speed_to_ratio(MAX_TICK_INTERVAL) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn speed_to_ratio_mid_value() {
        let mid = (MIN_TICK_INTERVAL + MAX_TICK_INTERVAL) / 2.0;
        let ratio = speed_to_ratio(mid);
        assert!((ratio - 0.5).abs() < 0.01);
    }

    #[test]
    fn speed_to_ratio_clamps_below_min() {
        assert!((speed_to_ratio(0.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn speed_to_ratio_clamps_above_max() {
        assert!((speed_to_ratio(10.0) - 0.0).abs() < f32::EPSILON);
    }

    // === ratio_to_speed テスト ===

    #[test]
    fn ratio_to_speed_0_returns_max_interval() {
        assert!((ratio_to_speed(0.0) - MAX_TICK_INTERVAL).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_to_speed_1_returns_min_interval() {
        assert!((ratio_to_speed(1.0) - MIN_TICK_INTERVAL).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_to_speed_clamps_negative() {
        assert!((ratio_to_speed(-0.5) - MAX_TICK_INTERVAL).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_to_speed_clamps_above_1() {
        assert!((ratio_to_speed(1.5) - MIN_TICK_INTERVAL).abs() < f32::EPSILON);
    }

    #[test]
    fn speed_roundtrip() {
        for &interval in &[0.1, 0.2, 0.5, 1.0, 1.5, 2.0] {
            let roundtrip = ratio_to_speed(speed_to_ratio(interval));
            assert!(
                (roundtrip - interval).abs() < 0.001,
                "roundtrip failed for {interval}: got {roundtrip}"
            );
        }
    }

    // === zoom_to_ratio テスト ===

    #[test]
    fn zoom_to_ratio_min_scale_returns_1() {
        // 最もズームイン（MIN_CAMERA_SCALE）は右端 = ratio 1.0
        assert!((zoom_to_ratio(MIN_CAMERA_SCALE) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn zoom_to_ratio_max_scale_returns_0() {
        // 最もズームアウト（MAX_CAMERA_SCALE）は左端 = ratio 0.0
        assert!((zoom_to_ratio(MAX_CAMERA_SCALE) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn zoom_to_ratio_mid_value() {
        let mid = (MIN_CAMERA_SCALE + MAX_CAMERA_SCALE) / 2.0;
        let ratio = zoom_to_ratio(mid);
        assert!((ratio - 0.5).abs() < 0.02);
    }

    #[test]
    fn zoom_to_ratio_clamps_below_min() {
        assert!((zoom_to_ratio(0.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn zoom_to_ratio_clamps_above_max() {
        assert!((zoom_to_ratio(2.0) - 0.0).abs() < f32::EPSILON);
    }

    // === ratio_to_zoom テスト ===

    #[test]
    fn ratio_to_zoom_0_returns_max_scale() {
        assert!((ratio_to_zoom(0.0) - MAX_CAMERA_SCALE).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_to_zoom_1_returns_min_scale() {
        assert!((ratio_to_zoom(1.0) - MIN_CAMERA_SCALE).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_to_zoom_clamps_negative() {
        assert!((ratio_to_zoom(-0.5) - MAX_CAMERA_SCALE).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_to_zoom_clamps_above_1() {
        assert!((ratio_to_zoom(1.5) - MIN_CAMERA_SCALE).abs() < f32::EPSILON);
    }

    #[test]
    fn zoom_roundtrip() {
        for &scale in &[0.05, 0.1, 0.2, 0.3] {
            let roundtrip = ratio_to_zoom(zoom_to_ratio(scale));
            assert!(
                (roundtrip - scale).abs() < 0.001,
                "roundtrip failed for {scale}: got {roundtrip}"
            );
        }
    }
}
