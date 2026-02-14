//! グリッドクリックとセルハイライトの処理

use bevy::prelude::*;

use crate::components::screen::CellHighlight;
use crate::events::PlayAudioEvent;
use crate::resources::{interaction::HoveredCell, world::World};
use crate::systems::coordinate::{screen_to_grid_coords, world_to_screen_pos};
use crate::WorldCamera;

/// グリッド上の左クリックを処理し、クリックされたセルをトグルする
pub fn handle_grid_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    mut world: ResMut<World>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, transform)) = camera_query.single() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(transform, cursor_pos) else {
        return;
    };
    if let Some((gx, gy)) = screen_to_grid_coords(world_pos, world.width, world.height) {
        world.toggle_cell(gx, gy);
    }
}

/// マウスカーソル位置に応じてセルハイライトを更新する
///
/// カーソルがグリッド上にある場合は該当セル位置にハイライトを表示し、
/// セルが変わった時に効果音を再生する。グリッド外では非表示にする。
pub fn update_cell_highlight(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    world: Res<World>,
    mut highlight_query: Query<(&mut Transform, &mut Visibility), With<CellHighlight>>,
    mut hovered: ResMut<HoveredCell>,
    mut events: MessageWriter<PlayAudioEvent>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_query.single() else {
        return;
    };

    let grid_coords = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(cam_transform, cursor_pos).ok())
        .and_then(|world_pos| screen_to_grid_coords(world_pos, world.width, world.height));

    let Ok((mut transform, mut vis)) = highlight_query.single_mut() else {
        return;
    };

    match grid_coords {
        Some((gx, gy)) => {
            *vis = Visibility::Inherited;
            let pos = world_to_screen_pos(gx, gy, world.width, world.height);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
            transform.translation.z = 1.0;

            let new_hover = Some((gx, gy));
            if hovered.0 != new_hover {
                hovered.0 = new_hover;
                events.write(PlayAudioEvent);
            }
        }
        None => {
            *vis = Visibility::Hidden;
            hovered.0 = None;
        }
    }
}
