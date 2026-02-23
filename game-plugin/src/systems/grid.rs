//! グリッドクリックとセルハイライトの処理

use bevy::prelude::*;
use common::consts::calc_viewport_sizes;

use crate::WorldCamera;
use crate::components::screen::CellHighlight;
use crate::resources::interaction::{DragState, HoveredCell};
use crate::resources::world::World;
use crate::systems::coordinate::{
    is_cursor_over_world_viewport, screen_to_grid_coords, world_to_screen_pos,
};

/// グリッド上の左クリックを処理し、クリックされたセルをトグルする
///
/// ドラッグ操作後のリリースではセルをトグルしない。
pub fn handle_grid_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    mut world: ResMut<World>,
    drag_state: Res<DragState>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return;
    }
    if drag_state.is_dragging {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let scale_factor = window.resolution.scale_factor();
    let sizes = calc_viewport_sizes(window.physical_width(), window.physical_height());
    if !is_cursor_over_world_viewport(cursor_pos, scale_factor, sizes.main_height) {
        return;
    }
    let Ok((camera, transform)) = camera_query.single() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(transform, cursor_pos) else {
        return;
    };
    let (gx, gy) = screen_to_grid_coords(world_pos);
    world.toggle_cell(gx, gy);
}

/// マウスカーソル位置に応じてセルハイライトを更新する
///
/// カーソルがワールドビューポート上にある場合は該当セル位置にハイライトを表示する。
/// ビューポート外では非表示にする。
pub fn update_cell_highlight(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    mut highlight_query: Query<(&mut Transform, &mut Visibility), With<CellHighlight>>,
    mut hovered: ResMut<HoveredCell>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_query.single() else {
        return;
    };

    let scale_factor = window.resolution.scale_factor();
    let sizes = calc_viewport_sizes(window.physical_width(), window.physical_height());

    let grid_coords = window
        .cursor_position()
        .filter(|&pos| is_cursor_over_world_viewport(pos, scale_factor, sizes.main_height))
        .and_then(|cursor_pos| camera.viewport_to_world_2d(cam_transform, cursor_pos).ok())
        .map(screen_to_grid_coords);

    let Ok((mut transform, mut vis)) = highlight_query.single_mut() else {
        return;
    };

    match grid_coords {
        Some((gx, gy)) => {
            *vis = Visibility::Inherited;
            let pos = world_to_screen_pos(gx, gy);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
            transform.translation.z = 1.0;

            hovered.0 = Some((gx, gy));
        }
        None => {
            *vis = Visibility::Hidden;
            hovered.0 = None;
        }
    }
}
