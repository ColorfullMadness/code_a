use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::MouseLoc;
use crate::game::player::components::*;
use crate::components::{ColliderBundle, Health, PlayerBundle};

use crate::components::{Weapon, Bullet, BulletBundle, Grenade, GrenadeBundle, DetonationTimer};
use crate::game::ui::AmmoCountText;
use crate::game::ui::HealthCountText;
use crate::graphics::*;


pub fn rotate_player(
    mouse_pos: ResMut<MouseLoc>,
    mut player_pos: Query<&mut Transform, With<Player>>,
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    for transform in &mut player_pos {
        if let Ok(mut sprite) = player_sprite.get_single_mut() {
            //println!("World position: {}/{}", world_position.x, world_position.y);
            if mouse_pos.loc.x < transform.translation.x && !sprite.flip_x {
                sprite.flip_x = true;
            } else if mouse_pos.loc.x > transform.translation.x && sprite.flip_x {
                sprite.flip_x = false;
            }
        }
    }
}

pub fn player_reload(
    mut weapon_query: Query<&mut Weapon, With<Player>>,
    input: Res<Input<KeyCode>>, 
    time: Res<Time>
){   
    if let Ok(mut weapon) = weapon_query.get_single_mut(){
        if (input.just_pressed(KeyCode::R) || input.pressed(KeyCode::R)) && weapon.ammo.bullets == 0{
            println!("RELOADING");
            weapon.reload_timer.reload_timer.tick(time.delta());
            if weapon.reload_timer.reload_timer.finished() {
                weapon.ammo.bullets = 300000;
                weapon.reload_timer.reload_timer.reset();
                println!("RELOADED + 30");
            }
        }
        if input.just_released(KeyCode::R) {
            weapon.reload_timer.reload_timer.reset();
        }
    }
}

pub fn player_shoot(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseLoc>,
    player_pos: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut weapon_query: Query<&mut Weapon, With<Player>>,
    time: Res<Time>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(mut weapon) = weapon_query.get_single_mut(){
            weapon.fire_rate.timer.reset();

            if weapon.ammo.bullets != 0{
                if let Ok(player_position) = player_pos.get_single() {

                    let bullet_velocity = (mouse_pos.loc - player_position.translation.truncate()).normalize();
                    let angle = bullet_velocity.y.atan2(bullet_velocity.x);
                    commands.spawn(BulletBundle {
                        sprite_bundle: SpriteBundle {
                            transform: Transform {
                                translation: Vec3::from_array([
                                    player_position.translation.x + bullet_velocity.x * 8.0,
                                    player_position.translation.y + bullet_velocity.y * 10.0,
                                    0.0,
                                ]),
                                rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle),
                                ..Default::default()
                            },
                            texture: asset_server.load("bullet.png"),
                            ..Default::default()
                        },
                        collider_bundle: ColliderBundle {
                            collider: Collider::cuboid(0.5, 1.5),
                            rigid_body: RigidBody::Dynamic,
                            velocity: Velocity::linear(bullet_velocity * 500.0),
                            ..Default::default()
                        },
                        bullet: Bullet {},
                    }).insert(Sensor).insert(ActiveEvents::COLLISION_EVENTS);

                    weapon.ammo.bullets -= 1;
                }
            }
        }
    } else if mouse_input.pressed(MouseButton::Left){ 
        //TODO add different actions for pressed and just pressed
        if let Ok(mut weapon) = weapon_query.get_single_mut(){
            weapon.fire_rate.timer.tick(time.delta());

            if weapon.fire_rate.to_owned().timer.finished() && weapon.ammo.bullets != 0{
                if let Ok(player_position) = player_pos.get_single() {

                    let bullet_velocity = (mouse_pos.loc - player_position.translation.truncate()).normalize();
                    let angle = bullet_velocity.y.atan2(bullet_velocity.x);
                    commands.spawn(BulletBundle {
                        sprite_bundle: SpriteBundle {
                            transform: Transform {
                                translation: Vec3::from_array([
                                    player_position.translation.x + bullet_velocity.x * 8.0,
                                    player_position.translation.y + bullet_velocity.y * 10.0,
                                    0.0,
                                ]),
                                rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle),
                                ..Default::default()
                            },
                            texture: asset_server.load("bullet.png"),
                            ..Default::default()
                        },
                        collider_bundle: ColliderBundle {
                            collider: Collider::cuboid(0.5, 1.5),
                            rigid_body: RigidBody::Dynamic,
                            velocity: Velocity::linear(bullet_velocity * 500.0),
                            ..Default::default()
                        },
                        bullet: Bullet {},
                    }).insert(Sensor).insert(ActiveEvents::COLLISION_EVENTS);

                    weapon.ammo.bullets -= 1;
                }
            }
        }
    }
}

pub fn player_throw_grenade(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mouse_pos: Res<MouseLoc>,
    player_pos: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if input.just_pressed(KeyCode::G) {
        if let Ok(player_position) = player_pos.get_single() {
            let bullet_velocity =
                (mouse_pos.loc - player_position.translation.truncate()).normalize();
            let angle = bullet_velocity.y.atan2(bullet_velocity.x);
            commands.spawn(GrenadeBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::from_array([
                            player_position.translation.x + bullet_velocity.x * 8.0,
                            player_position.translation.y + bullet_velocity.y * 10.0,
                            0.0,
                        ]),
                        rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle),
                        ..Default::default()
                    },
                    texture: asset_server.load("granade.png"),
                    ..Default::default()
                },
                collider_bundle: ColliderBundle {
                    collider: Collider::cuboid(0.5, 1.5),
                    rigid_body: RigidBody::Dynamic,
                    velocity: Velocity::linear(bullet_velocity * 100.0),
                    ..Default::default()
                },
                timer: DetonationTimer {
                    detonation_timer: Timer::from_seconds(2.0, TimerMode::Once),
                },
                grenade: Grenade,
            });
        }
    }
}

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    mut player_anim: Query<&mut Animations, With<Player>>,
) {
    for mut velocity in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
        let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 150.;
        velocity.linvel.y = (up - down) * 150.;

        if !velocity.eq(&Velocity::zero()) {
            if let Ok(mut anim) = player_anim.get_single_mut() {
                anim.current_animation = 0;
            }
        }
    }
}

pub fn talk(
    input: Res<Input<KeyCode>>,
    characters: Res<CharacterSheet>,
    mut player_anim: Query<&mut Animations, With<Player>>,
) {
    if input.just_pressed(KeyCode::T) {
        if let Ok(mut animation) = player_anim.get_single_mut() {
            animation.current_animation = 1;
            dbg!(animation);
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    _ev_asset: EventReader<AssetEvent<Image>>,
    _asset_server: Res<AssetServer>,
    spawn_query: Query<&GridCoords, With<Spawn>>,
    player_query: Query<Entity, With<Player>>,
    _assets: Res<Assets<Image>>,
    characters: Res<CharacterSheet>,
) {
    if player_query.is_empty() {
        println!("Creating players");
        println!("{:?}", spawn_query);

        spawn_query.for_each(|cords| {
            println!("1Spawning player at cords: x:{}, y:{}", cords.x, cords.y);
            commands
                .spawn(PlayerBundle {
                    sprite_bundle: SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            (cords.x * 16 + 8) as f32,
                            (cords.y * 16 + 8) as f32,
                            0.0,
                        ),
                        sprite: TextureAtlasSprite::new(characters.run_animation[0]),
                        texture_atlas: characters.handle.clone(),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    collider_bundle: ColliderBundle {
                        collider: Collider::cuboid(5.0, 5.0),
                        rigid_body: RigidBody::Dynamic,
                        friction: Friction {
                            coefficient: 0.0,
                            combine_rule: CoefficientCombineRule::Min,
                        },
                        rotation_constraints: LockedAxes::ROTATION_LOCKED,
                        ..Default::default()
                    },
                    weapon: Weapon {
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Animations {
                    animations: vec![
                        FrameAnimation {
                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                            frames: characters.run_animation.to_vec(),
                            current_frame: 0,
                        },
                        FrameAnimation {
                            timer: Timer::from_seconds(0.4, TimerMode::Repeating),
                            frames: characters.talk_animation.to_vec(),
                            current_frame: 0,
                        },
                    ],
                    current_animation: 0,
                });
        });
    }
}

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