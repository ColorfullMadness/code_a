use bevy::prelude::*;

mod systems;
pub mod components;

use systems::*;

use crate::AppState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                zombie_movement, 
                despawn_zombie
            )
            .in_set(OnUpdate(AppState::Game))
            //.in_set(OnUpdate(SimulationState::Running))
        )
        .add_systems(
            (
                despawn_zombies,
                )
                .in_schedule(OnEnter(AppState::GameOver))
        );
    }
}