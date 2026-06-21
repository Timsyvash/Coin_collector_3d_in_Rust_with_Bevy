use bevy::prelude::*;
use crate::coin::coin::*;
use crate::game::win::*;
use crate::player::player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (collision_player_with_coins, check_win_system).run_if(
            in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Win), win_function);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Win,
}

fn collision_player_with_coins(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    mut count: ResMut<Count>
) {
    let Ok(player_transform) = player_query.get_single() else { return; };

    for (coin_entity, coin_transform) in coin_query.iter() {
        let distance = player_transform.translation.distance(coin_transform.translation);

        if distance < 1.0 {
            count.value += 1;
            commands.entity(coin_entity).despawn_recursive();
        }
    }
}