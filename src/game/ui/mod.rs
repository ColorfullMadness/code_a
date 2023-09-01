use bevy::prelude::*;

pub mod hud;

pub use hud::HudPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(HudPlugin);
    }
}