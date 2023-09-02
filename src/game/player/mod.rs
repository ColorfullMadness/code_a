use bevy::prelude::*;

use self::systems::*;
use crate::AppState;

use systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(Update, 
            (
                player_movement, 
                player_reload, 
                player_shoot, 
                player_throw_grenade,
                rotate_player,
            )
        );
    }
}