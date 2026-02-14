//! ウィンドウリサイズ時のビューポート更新システム

use bevy::{camera::Viewport, prelude::*};
use common::consts::calc_viewport_sizes;

use crate::components::camera::{BottomPanelCamera, WorldCamera};

/// ウィンドウサイズに応じて両カメラのビューポートを更新するシステム
pub fn update_camera_viewports(
    windows: Query<&Window>,
    mut world_camera: Query<&mut Camera, (With<WorldCamera>, Without<BottomPanelCamera>)>,
    mut panel_camera: Query<&mut Camera, (With<BottomPanelCamera>, Without<WorldCamera>)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let sizes = calc_viewport_sizes(window.physical_width(), window.physical_height());

    if let Ok(mut camera) = world_camera.single_mut() {
        camera.viewport = Some(Viewport {
            physical_position: [0, 0].into(),
            physical_size: [sizes.viewport_width, sizes.main_height].into(),
            ..default()
        });
    }
    if let Ok(mut camera) = panel_camera.single_mut() {
        camera.viewport = Some(Viewport {
            physical_position: [0, sizes.main_height].into(),
            physical_size: [sizes.viewport_width, sizes.panel_height].into(),
            ..default()
        });
    }
}
