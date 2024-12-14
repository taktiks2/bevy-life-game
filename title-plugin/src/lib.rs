use bevy::{
    color::palettes::css::*,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

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
        app.add_systems(
            Update,
            (title_action, title_input).run_if(in_state(GameState::Title)),
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

#[derive(Component)]
pub struct TestParent;

fn setup_title_screen(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Transform::default(),
            BackgroundColor(WHITE.into()),
            TestParent,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    ..default()
                },
                BackgroundColor(BLACK.into()),
            ));
        });

    // commands
    //     .spawn((
    //         Node {
    //             flex_direction: FlexDirection::Column,
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::SpaceBetween,
    //             padding: UiRect {
    //                 left: Val::Px(0.),
    //                 right: Val::Px(0.),
    //                 top: Val::Px(200.),
    //                 bottom: Val::Px(200.),
    //             },
    //             width: Val::Percent(100.),
    //             height: Val::Percent(100.),
    //             ..default()
    //         },
    //         BackgroundColor(WHITE.into()),
    //         OnTitleScreen,
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Text::new("Conway's Game of Life"),
    //             TextFont {
    //                 font: game_assets.font_bold.clone(),
    //                 font_size: 60.0,
    //                 ..default()
    //             },
    //             TextColor(BLACK.into()),
    //         ));
    //         parent
    //             .spawn((
    //                 Node {
    //                     align_items: AlignItems::Center,
    //                     justify_content: JustifyContent::Center,
    //                     width: Val::Px(200.),
    //                     height: Val::Px(60.),
    //                     ..default()
    //                 },
    //                 Button,
    //                 TitleButtonAction::Start,
    //                 BorderRadius::px(5., 5., 5., 5.),
    //                 BackgroundColor(BLACK.into()),
    //             ))
    //             .with_children(|p| {
    //                 p.spawn((
    //                     Text::new("Start"),
    //                     TextFont {
    //                         font: game_assets.font_bold.clone(),
    //                         font_size: 40.0,
    //                         ..default()
    //                     },
    //                 ));
    //             });
    //     });
}

pub fn title_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut parent_query: Query<&mut Transform, With<TestParent>>,
    mut camera_query: Query<&mut Transform, (With<TitleCamera>, Without<TestParent>)>,
) {
    if keys.just_pressed(KeyCode::KeyW) {
        println!("KeyW pressed");
        for mut transform in camera_query.iter_mut() {
            transform.translation.y += 100.;
        }
    }
    if keys.just_pressed(KeyCode::KeyA) {
        println!("KeyA pressed");
        for mut transform in camera_query.iter_mut() {
            transform.translation.x -= 100.;
        }
    }
    if keys.just_pressed(KeyCode::KeyD) {
        println!("KeyD pressed");
        for mut transform in camera_query.iter_mut() {
            transform.translation.x += 100.;
        }
    }
    if keys.just_pressed(KeyCode::KeyS) {
        println!("KeyS pressed");
        for mut transform in camera_query.iter_mut() {
            transform.translation.y -= 100.;
        }
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        println!("KeyQ pressed");
        for mut transform in parent_query.iter_mut() {
            transform.scale -= Vec3::new(0.1, 0.1, 0.0); // スケールを減少
        }
    }
    if keys.just_pressed(KeyCode::KeyE) {
        println!("KeyE pressed");
        for mut transform in parent_query.iter_mut() {
            transform.scale += Vec3::new(0.1, 0.1, 0.0); // スケールを増加
        }
    }
}

// pub fn title_mouse_input(
//     windows: Query<&Window, With<PrimaryWindow>>,
//     mut mouse_input: EventReader<MouseButtonInput>,
//     mut query: Query<(&mut Transform, With<TitleCamera>)>,
// ) {
//     let window = windows.single();
//     for event in mouse_input.read() {
//         if let ButtonState::Pressed = event.state {
//             let position = window.cursor_position();
//             for event in query.iter_mut() {
//                 println!("Mouse clicked: {:?}", position);
//                 // event.scale += Vec3::new(0.1, 0.1, 0.0);
//             }
//         }
//     }
// }

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
