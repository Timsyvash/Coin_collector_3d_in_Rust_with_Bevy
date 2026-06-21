use bevy::prelude::*;
use crate::coin::coin::*;
use crate::game::game::*;

#[derive(Component)]
pub struct Win;

pub fn win_function(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        Win,
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Ви перемогли",
                TextStyle {
                    font: asset_server.load("FiorinaTitle-Light.otf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        });
    });
}

pub fn check_win_system(
    count: Res<Count>,
    mut next_state: ResMut<NextState<GameState>>,
    max_count: Res<MaxCount>
) {
    if count.value >= max_count.max_count {
        next_state.set(GameState::Win);
    }
}