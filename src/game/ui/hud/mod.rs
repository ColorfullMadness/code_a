use bevy::prelude::*;

mod layout;
mod styles;

use crate::game::ui::hud::layout::*;

use crate::AppState;

#[derive(Component)]
pub struct Hud {}

pub struct HudPlugin; 

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)));
    }
}