use bevy::prelude::*;

use crate::components::{Weapon, Health};
use crate::game::player::components::Player;
use crate::game::ui::hud::{AmmoCountText, HealthCountText};

pub fn update_ammo_text(
    weapon_query: Query<&Weapon, With<Player>>,
    mut text_query: Query<&mut Text, With<AmmoCountText>>,
) {
    if let Ok(weapon) = weapon_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Ammo: {}", weapon.ammo.bullets);
        }
    }
}

pub fn update_health_text(
    health_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthCountText>>,
) {
    if let Ok(health) = health_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Health: {}", health.health_points);
        }
    }
}