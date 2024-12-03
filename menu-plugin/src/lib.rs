use bevy::{color::palettes::css::*, prelude::*};

use common::{states::GameState, systems::despawn_screen};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup);
        app.add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
        app.add_systems(
            Update,
            (menu_action, menu_input_keyboard_handling).run_if(in_state(GameState::Menu)),
        );
    }
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Back,
    Quit,
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
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
                            ..default()
                        },
                        MenuButtonAction::Back,
                        BackgroundColor(BLACK.into()),
                        BorderRadius::px(5., 5., 5., 5.),
                        Button,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Back"),
                            TextFont {
                                font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
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
                            ..default()
                        },
                        MenuButtonAction::Quit,
                        BackgroundColor(BLACK.into()),
                        BorderRadius::px(5., 5., 5., 5.),
                        Button,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Quit"),
                            TextFont {
                                font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
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
    mut app_exit_events: EventWriter<AppExit>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, title_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match title_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
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
    if keys.just_pressed(KeyCode::KeyQ) {
        state.set(GameState::Game);
    }
}
