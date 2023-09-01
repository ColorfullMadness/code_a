use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::player::components::Player;
use crate::components::Health;
use super::components::*;

pub fn zombie_movement(
    mut zombie_query: Query<(&mut Velocity, &Transform), With<Zombie>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        for (mut zombie_vel, zombie_pos) in zombie_query.iter_mut() {
            if zombie_pos.translation.distance(player_pos.translation) < 50.0 {
                zombie_vel.linvel = (player_pos.translation - zombie_pos.translation)
                    .truncate()
                    .normalize()
                    * 50.0;
            } else {
                zombie_vel.linvel = Vec2::ZERO;
            }
        }
    }
}

pub fn despawn_zombie(
    mut commands: Commands, 
    zombie_query: Query<(&mut Health, Entity), With<Zombie>>
) {
    for (health, zombie) in zombie_query.iter() {
        if health.health_points <= 0 {
            commands.entity(zombie).despawn();
        }
    }
}