use bevy::prelude::*;

pub mod hud;

pub use hud::HudPlugin;

pub struct GameUIPlugin;

#[derive(Component)]
pub struct AmmoCountText;

#[derive(Component)]
pub struct HealthCountText;


impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(HudPlugin);
    }
}