use bevy::camera::visibility::RenderLayers;

#[derive(Clone, Copy)]
pub enum Layer {
    SideMenu,
    World,
}

impl Layer {
    pub fn as_render_layer(self) -> RenderLayers {
        RenderLayers::layer(self as usize)
    }
}
