use bevy::{color::palettes::css::*, prelude::*};

use common::{
    resources::GameAssets,
    states::GameState,
    systems::{despawn_entity, setup_camera},
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (setup_menu_screen, setup_menu_camera),
        );
        app.add_systems(
            OnExit(GameState::Menu),
            (despawn_entity::<OnMenuScreen>, despawn_entity::<MenuCamera>),
        );
        app.add_systems(
            Update,
            (menu_action, menu_input_keyboard_handling).run_if(in_state(GameState::Menu)),
        );
    }
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct MenuCamera;

#[derive(Component)]
enum MenuButtonAction {
    Back,
    Quit,
}

fn setup_menu_camera(commands: Commands) {
    setup_camera(commands, MenuCamera);
}

fn setup_menu_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(200.),
                    bottom: Val::Px(200.),
                },
                ..default()
            },
            OnMenuScreen,
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font: game_assets.font_bold.clone(),
                    font_size: 60.0,
                    ..default()
                },
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Px(200.),
                    height: Val::Px(200.),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(200.),
                            height: Val::Px(60.),
                            border_radius: BorderRadius::px(5., 5., 5., 5.),
                            ..default()
                        },
                        MenuButtonAction::Back,
                        BackgroundColor(BLACK.into()),
                        Button,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Back"),
                            TextFont {
                                font: game_assets.font_bold.clone(),
                                font_size: 40.0,
                                ..default()
                            },
                        ));
                    });
                    p.spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(200.),
                            height: Val::Px(60.),
                            border_radius: BorderRadius::px(5., 5., 5., 5.),
                            ..default()
                        },
                        MenuButtonAction::Quit,
                        BackgroundColor(BLACK.into()),
                        Button,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Quit"),
                            TextFont {
                                font: game_assets.font_bold.clone(),
                                font_size: 40.0,
                                ..default()
                            },
                        ));
                    });
                });
        });
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: MessageWriter<AppExit>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, title_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match title_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Back => {
                    state.set(GameState::Title);
                }
            }
        }
    }
}

pub fn menu_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        state.set(GameState::Game);
    }
}
