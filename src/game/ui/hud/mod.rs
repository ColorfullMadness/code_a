use bevy::prelude::*;

mod layout;
mod styles;
mod systems;

use crate::AppState;
use crate::game::ui::hud::layout::spawn_hud;
use super::hud::systems::*;

#[derive(Component)]
pub struct Hud {}

#[derive(Component)]
pub struct AmmoCountText;

#[derive(Component)]
pub struct HealthCountText;
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            .add_systems((update_ammo_text, update_health_text).in_set(OnUpdate(AppState::Game)))
            .add_system(update_health_text.in_set(OnUpdate(AppState::Game)));
    }
}