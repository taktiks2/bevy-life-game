use bevy::prelude::States;

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    Title,
    Game,
    Menu,
}
