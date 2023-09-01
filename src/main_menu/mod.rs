use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use crate::AppState;

use self::systems::{layout::{spawn_main_menu, despawn_main_menu}, interactions::{interact_with_play_button, interact_with_quit_button}};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        //OnEnter State Systems
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        //Systems
        .add_systems(
            (
                interact_with_play_button,
                interact_with_quit_button
            ).in_set(OnUpdate(AppState::MainMenu))
        )
        //OnExit State Systems
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }    
}