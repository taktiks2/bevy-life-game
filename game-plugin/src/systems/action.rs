use bevy::prelude::*;

use crate::components::action::GameButtonAction;

#[allow(clippy::type_complexity)]
pub fn game_action(
    interaction_query: Query<
        (&Interaction, &GameButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                GameButtonAction::Start => {
                    println!("Start");
                }
                GameButtonAction::Stop => {
                    println!("Stop");
                }
                GameButtonAction::Next => {
                    println!("Next");
                }
                GameButtonAction::Reset => {
                    println!("Reset");
                }
            }
        }
    }
}
