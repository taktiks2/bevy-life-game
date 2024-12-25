pub const WINDOW_WIDTH: f32 = 1000.;
pub const WINDOW_HEIGHT: f32 = 800.;
pub const SUB_PHYSICAL_WIDTH: u32 = (WINDOW_WIDTH * 0.2) as u32;
pub const MAIN_PHYSICAL_WIDTH: u32 = (WINDOW_WIDTH * 0.8) as u32;
pub const INTERVAL_WIDTH: f32 = (MAIN_PHYSICAL_WIDTH / WORLD_WIDTH as u32) as f32;
pub const INTERVAL_HEIGHT: f32 = WINDOW_HEIGHT / WORLD_HEIGHT as f32;
pub const CELL_WIDTH: f32 = INTERVAL_WIDTH * 0.95;
pub const CELL_HEIGHT: f32 = INTERVAL_HEIGHT * 0.95;
pub const OFFSET_WIDTH: f32 = MAIN_PHYSICAL_WIDTH as f32 / 2. - INTERVAL_WIDTH / 2.;
pub const OFFSET_HEIGHT: f32 = WINDOW_HEIGHT / 2. - INTERVAL_HEIGHT / 2.;
pub const PHYSICAL_HEIGHT: u32 = (WINDOW_HEIGHT as u32) * 2;
pub const WORLD_WIDTH: u16 = 100;
pub const WORLD_HEIGHT: u16 = 100;
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
