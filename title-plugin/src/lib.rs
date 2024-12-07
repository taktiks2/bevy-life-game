use bevy::{color::palettes::css::*, prelude::*};

use common::{
    resources::GameAssets,
    states::GameState,
    systems::{despawn_entity, setup_camera},
};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Title),
            (setup_title_screen, setup_title_camera),
        );
        app.add_systems(
            OnExit(GameState::Title),
            (
                despawn_entity::<OnTitleScreen>,
                despawn_entity::<TitleCamera>,
            ),
        );
        app.add_systems(Update, (title_action).run_if(in_state(GameState::Title)));
    }
}

#[derive(Component)]
struct OnTitleScreen;

#[derive(Component)]
struct TitleCamera;

#[derive(Component)]
enum TitleButtonAction {
    Start,
}

fn setup_title_camera(commands: Commands) {
    setup_camera(commands, TitleCamera);
}

fn setup_title_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(200.),
                    bottom: Val::Px(200.),
                },
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(WHITE.into()),
            OnTitleScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Conway's Game of Life"),
                TextFont {
                    font: game_assets.font_bold.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(BLACK.into()),
            ));
            parent
                .spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    Button,
                    TitleButtonAction::Start,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Start"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                    ));
                });
        });
}

#[allow(clippy::type_complexity, irrefutable_let_patterns)]
fn title_action(
    interaction_query: Query<
        (&Interaction, &TitleButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, title_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let TitleButtonAction::Start = title_button_action {
                state.set(GameState::Game);
            }
        }
    }
}
