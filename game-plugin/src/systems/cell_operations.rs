use bevy::{color::palettes::css::*, prelude::*};

use crate::components::{coordinate::Coordinate, screen::OnGameScreen};
use crate::events::ProgressGenerationEvent;
use crate::resources::world::World;

pub fn spawn_cells(mut commands: Commands, world: Res<World>) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                grid_template_columns: RepeatedGridTrack::auto(world.width),
                grid_template_rows: RepeatedGridTrack::auto(world.height),
                row_gap: Val::Px(1.),
                column_gap: Val::Px(1.),
                padding: UiRect::all(Val::Px(1.)),
                ..default()
            },
            BackgroundColor(GRAY.into()),
            OnGameScreen,
        ))
        .with_children(|parent| {
            for (y, row) in world.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    parent.spawn((
                        Button,
                        Coordinate {
                            x: x as u16,
                            y: y as u16,
                        },
                        // NOTE: interactionで認識ができるようにButtonBundleにする
                        Node {
                            display: Display::Grid,
                            width: Val::Auto,
                            height: Val::Auto,
                            ..default()
                        },
                        BackgroundColor(cell.get_color()),
                        BorderRadius::px(5., 5., 5., 5.),
                    ));
                }
            }
        });
}

// NOTE: interaction_queryのwarningを出さないようにするには、以下の型をあてること
// type InteractionQuery<'a, 'b> =
//     Query<'a, 'b, (&'a Interaction, &'b Coordinate), (Changed<Interaction>, With<Button>)>;

#[allow(clippy::type_complexity)]
pub fn switch_cell_state(
    mut interaction_query: Query<(&Interaction, &Coordinate), (Changed<Interaction>, With<Button>)>,
    mut world: ResMut<World>,
) {
    for (interaction, coordinate) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            world.cells[coordinate.y as usize][coordinate.x as usize] =
                world.cells[coordinate.y as usize][coordinate.x as usize].switch_state()
        }
    }
}

pub fn update_cells(world: Res<World>, mut query: Query<(&Coordinate, &mut BackgroundColor)>) {
    for (coordinate, mut background_color) in query.iter_mut() {
        let cell = &world.cells[coordinate.y as usize][coordinate.x as usize];
        *background_color = BackgroundColor(cell.get_color());
    }
}

pub fn progress_generation(
    mut world: ResMut<World>,
    mut progress_generation_event_reader: EventReader<ProgressGenerationEvent>,
) {
    for _ in progress_generation_event_reader.read() {
        world.progress_generation()
    }
}
