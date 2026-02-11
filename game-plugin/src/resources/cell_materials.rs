use bevy::{
    color::palettes::css,
    prelude::{Assets, ColorMaterial, Handle, Resource},
};

#[derive(Resource)]
pub struct CellMaterials {
    pub alive: Handle<ColorMaterial>,
    pub dead: Handle<ColorMaterial>,
}

impl CellMaterials {
    pub fn new(materials: &mut Assets<ColorMaterial>) -> Self {
        Self {
            alive: materials.add(ColorMaterial::from_color(css::BLACK)),
            dead: materials.add(ColorMaterial::from_color(css::WHITE)),
        }
    }
}
