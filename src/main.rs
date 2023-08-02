use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod systems;
mod graphics;

use graphics::GraphicsPlugin;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LdtkPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(GraphicsPlugin)
        // Required to prevent race conditions between bevy_ecs_ldtk's and bevy_rapier's systems
        .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { 
                load_level_neighbors: false 
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        }) 
        .insert_resource(MouseLoc{ loc: Vec2::ZERO})
        .add_startup_system(setup)
        //.add_system(coursor_pos)
        .add_system(bullet_collisions)
        .add_system(spawn_buddy)
        .add_system(player_reload)
        .add_system(mouse_movement_updating_system)
        .add_system(rotate_player)
        .add_system(spawn_player)
        .add_system(spawn_wall_collision)
        .add_system(zombie_movement)
        .add_system(player_movement)
        .add_system(player_shoot)
        .add_system(player_throw_grenade)
        .add_system(blow_up_granade)
        .add_system(despawn_zombie)
        //.add_system(move_bullets)
        .add_system(camera_fit_inside_current_level)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::SpawnBundle>(2)
        //.register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ZombieBundle>("Zombie")
        .run();
}
