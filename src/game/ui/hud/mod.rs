use bevy::prelude::*;

mod layout;
mod styles;

use crate::game::player::systems::{update_ammo_text, update_health_text}; // Assuming update_ammo_text is made public.
use crate::AppState;
use crate::game::ui::hud::layout::spawn_hud;

#[derive(Component)]
pub struct Hud {}

#[derive(Component)]
pub struct AmmoCountText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            .add_system(update_ammo_text.in_set(OnUpdate(AppState::Game)))
            .add_system(update_health_text.in_set(OnUpdate(AppState::Game)));
    }
}