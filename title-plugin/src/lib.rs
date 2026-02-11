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
    }
}

#[derive(Component)]
struct OnTitleScreen;

#[derive(Component)]
pub struct TitleCamera;

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
                        border_radius: BorderRadius::px(5., 5., 5., 5.),
                        ..default()
                    },
                    Button,
                    TitleButtonAction::Start,
                    BackgroundColor(BLACK.into()),
                ))
                .observe(on_start_button_click)
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

fn on_start_button_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Game);
}
