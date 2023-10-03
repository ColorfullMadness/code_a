use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::thread;
use std::time::Duration;

use crate::MouseLoc;
use crate::game::player::components::*;
use crate::components::{ColliderBundle, Health, PlayerBundle, Ammo};

use crate::components::{Weapon, Bullet, BulletBundle, Grenade, GrenadeBundle, DetonationTimer};
use crate::game::enemies::components::Zombie;
use crate::graphics::*;
use crate::AppState;

//TODO add another system that drives player animations

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

pub fn player_take_dmg(
    mut zombies: Query<(&Transform, Entity), With<Zombie>>,
    mut player: Query<(&mut Health, Entity, &Transform, &mut Velocity), With<Player>>,
    mut player_collisions: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for col_event in player_collisions.iter() {
        println!("Received collision event: {:?}", col_event.to_owned());
        match col_event.to_owned() {
            CollisionEvent::Started(e1, e2, _) => {
                for (zombie_transform, zombie_entity) in zombies.iter() {
                    for (mut health, player_entity, mut player_transform, mut velocity) in player.iter_mut() {
                        if player_entity.eq(&e1) || player_entity.eq(&e2) {
                            if zombie_entity.eq(&e1) || zombie_entity.eq(&e2) {
                                health.health_points -= 1;
                                println!(
                                    "Player: {:?} took 1 dmg and now has: {:?}",
                                    commands.entity(player_entity).id(),
                                    health.health_points
                                );

                                velocity.linvel += (zombie_transform.translation
                                    + player_transform.translation)
                                    .truncate()
                                    .normalize()
                                    * 500.0;
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _) => {}
        }
    }
}

pub fn kill_player(
    player_health: Query<(Entity, &Health), With<Player>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut commands: Commands
) {
    if let Ok((player, health)) = player_health.get_single() {
        if health.health_points == 0{
            app_state_next_state.set(AppState::GameOver);
            commands.entity(player).despawn();
            //std::thread::sleep(Duration::new(2,0) );
            //app_state_next_state.set(AppState::MainMenu);
        }
    }
}

pub fn player_reload(
    mut weapon_query: Query<&mut Weapon, With<Player>>,
    input: Res<Input<KeyCode>>, 
    time: Res<Time>
){   
    if let Ok(mut weapon) = weapon_query.get_single_mut(){
        if input.just_pressed(KeyCode::R) {
            println!("RELOADING");
            weapon.reloading = true;
        }
        if weapon.reloading {
            weapon.reload_timer.reload_timer.tick(time.delta());
        }
        if weapon.reload_timer.reload_timer.finished() {
            weapon.ammo.bullets = weapon.mag_size;
            weapon.reload_timer.reload_timer.reset();
            weapon.reloading = false;
            println!("RELOADED + 30");
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
    mut player_anim: Query<&mut Animations, With<Player>>,
) {

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(mut weapon) = weapon_query.get_single_mut(){
            weapon.fire_rate.timer.reset();

            if weapon.ammo.bullets != 0{
                if let Ok(mut anim) = player_anim.get_single_mut() {
                    anim.current_animation = 2;
                }
    
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
        if let Ok(mut weapon) = weapon_query.get_single_mut(){
            weapon.fire_rate.timer.tick(time.delta());
            if weapon.ammo.bullets != 0{
                if let Ok(mut anim) = player_anim.get_single_mut() {
                    anim.current_animation = 2;
                }

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
    player_weapon: Query<&Weapon, With<Player>>,
    mut player_anim: Query<&mut Animations, With<Player>>,
) {
    for mut velocity in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
        let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 150.;
        velocity.linvel.y = (up - down) * 150.;

        if let Ok(weapon) = player_weapon.get_single() {
            if weapon.reloading {
                velocity.linvel.x /= 2.0;
                velocity.linvel.y /= 2.0;
            }
        }

        if let Ok(mut anim) = player_anim.get_single_mut() {
            if !velocity.eq(&Velocity::zero()) {
                anim.current_animation = 0;
            } else {
                anim.current_animation = 3;
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
            let mut x: f32 = cords.x as f32 * 16.0;
            let mut y: f32 = cords.y as f32 * 16.0;
            x += 8.0;
            y += 8.0;
            y *= 2f32;
            commands
                .spawn(PlayerBundle {
                    sprite_bundle: SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            x,
                            y,
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
                        reloading: false,
                        mag_size: 30,
                        ammo: Ammo{
                            bullets: 0,
                        },
                        ..Default::default()
                    },
                    health: Health{
                        health_points: 10
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
                        FrameAnimation {
                            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                            frames: characters.shoot.to_vec(),
                            current_frame: 0,
                        },
                        FrameAnimation {
                            timer: Timer::from_seconds(0.4, TimerMode::Once),
                            frames: characters.idle.to_vec(),
                            current_frame: 0,
                        }
                    ],
                    current_animation: 0,
                })
                .insert(ActiveEvents::COLLISION_EVENTS);
        });
    }
}