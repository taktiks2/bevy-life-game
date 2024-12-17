pub const WINDOW_WIDTH: f32 = 1000.;
pub const WINDOW_HEIGHT: f32 = 800.;
pub const SUB_PHYSICAL_WIDTH: u32 = (WINDOW_WIDTH * 0.2) as u32 * 2;
pub const MAIN_PHYSICAL_WIDTH: u32 = (WINDOW_WIDTH * 0.8) as u32 * 2;
pub const PHYSICAL_HEIGHT: u32 = (WINDOW_HEIGHT as u32) * 2;
pub const WORLD_WIDTH: u16 = 4;
pub const WORLD_HEIGHT: u16 = 4;
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
