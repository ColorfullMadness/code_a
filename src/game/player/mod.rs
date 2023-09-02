use bevy::prelude::*;

use self::systems::*;
use crate::AppState;

use systems::*;

pub mod components;
pub(crate) mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_systems(
            (
                player_movement, 
                player_reload, 
                player_shoot, 
                player_throw_grenade,
                rotate_player,
                talk,
            ).in_set(OnUpdate(AppState::Game))
        );
    }
}