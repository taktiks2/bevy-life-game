use bevy::prelude::{Font, Handle, Resource};

#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    pub font_regular: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub test: String,
}
