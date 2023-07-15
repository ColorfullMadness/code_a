use crate::components::*;
use bevy::{prelude::*, transform::commands};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use std::collections::{HashMap, HashSet};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn((camera, MainCamera));

    let ldtk_handle = asset_server.load("test.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

#[derive(Resource)]
pub struct MouseLoc {
    pub loc: Vec2,
}

pub fn mouse_movement_updating_system(
    mut mouse_pos: ResMut<MouseLoc>,
    mut mouse_pos_event: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for e in mouse_pos_event.iter() {
        if let Ok((camera, cam_transform)) = camera_query.get_single() {
            let mouse = Vec2::from_array([e.position.x, e.position.y]);
            if let Some(world_position) = camera.viewport_to_world_2d(cam_transform, mouse) {
                mouse_pos.loc = world_position;
                //println!("MOUSE at: {}/{}", mouse_pos.loc.x, mouse_pos.loc.y);
            }
        }
    }
}

pub fn rotate_player(
    mouse_pos: ResMut<MouseLoc>,
    mut player_pos: Query<&mut Transform, With<Player>>,
    mut player_sprite: Query<&mut Sprite, With<Player>>,
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

pub fn zombie_movement(
    mut zombie_query: Query<(&mut Velocity, &Transform), With<Zombie>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        for (mut zombie_vel, zombie_pos) in zombie_query.iter_mut() {
            if zombie_pos.translation.distance(player_pos.translation) < 50.0 {
                zombie_vel.linvel = (player_pos.translation - zombie_pos.translation)
                    .truncate()
                    .normalize()
                    * 50.0;
            } else {
                zombie_vel.linvel = Vec2::ZERO;
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
                weapon.ammo.bullets = 30;
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

    if mouse_input.pressed(MouseButton::Left) || mouse_input.just_pressed(MouseButton::Left) { 
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

pub fn blow_up_granade(
    time: Res<Time>,
    mut grenades: Query<(&mut DetonationTimer, &Transform, Entity), With<Grenade>>,
    mut commands: Commands,
    mut zombies: Query<(&Transform, &mut Health), With<Zombie>>,
) {
    for (mut det_timer, grenade_transform, entity) in grenades.iter_mut() {
        det_timer.detonation_timer.tick(time.delta());
        if det_timer.detonation_timer.finished() {
            commands.entity(entity).despawn();
            for (zombie_trans, mut zombie_health) in zombies.iter_mut() {
                if zombie_trans.translation.distance(grenade_transform.translation) < 50.0 {
                    zombie_health.health_points -= 10;
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

                    let bullet_velocity = (mouse_pos.loc - player_position.translation.truncate()).normalize();
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
                        timer: DetonationTimer {detonation_timer: Timer::from_seconds(2.0, TimerMode::Once) }, 
                        grenade: Grenade, 
                    });

                }
    }
}


pub fn despawn_zombie(
    mut commands: Commands, 
    zombie_query: Query<(&mut Health, Entity), With<Zombie>>
) {
    for (health, zombie) in zombie_query.iter() {
        if health.health_points == 0 {
            commands.entity(zombie).despawn();
        }
    }
}

pub fn bullet_collisions(
    mut bullet_collisions: EventReader<CollisionEvent>, 
    mut zombie_query: Query<(&mut Health, Entity, &mut Velocity, &Transform), With<Zombie>>,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    mut commands: Commands,
){
    for bullet in bullet_collisions.iter(){
        println!("Received collision event: {:?}", bullet.to_owned());
        let b = bullet.to_owned();
        match b {
            CollisionEvent::Started(e1, e2, _) => {
                for (bullet_transform, bullet_entity) in bullet_query.iter() {
                    for (mut health, zombie_entity, mut zombie_vel, zombie_transform) in zombie_query.iter_mut() {

                        if bullet_entity.eq(&e2) {
                            commands.entity(e2).despawn();
                            if zombie_entity.eq(&e1){
                                health.health_points -= 1;
                                println!("Entity: {:?} took 1 dmg and now has: {:?}", commands.entity(zombie_entity).id(), health.health_points);

                                zombie_vel.linvel += (bullet_transform.translation + zombie_transform.translation)
                                    .truncate()
                                    .normalize()
                                    * 500.0;
                            }
                        } else if bullet_entity.eq(&e1) {
                            commands.entity(e1).despawn();
                            if zombie_entity.eq(&e2){
                                health.health_points -= 1;
                                println!("Entity: {:?} took 1 dmg and now has: {:?}", commands.entity(zombie_entity).id(), health.health_points);

                                zombie_vel.linvel += (bullet_transform.translation + zombie_transform.translation)
                                    .truncate()
                                    .normalize()
                                    * 500.0;
                            }
                        }
                    }
                }
                
            }, 
            CollisionEvent::Stopped(_e1, _e2, _) => {
             
            }
        }
    }
}

pub fn player_movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
        let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 150.;
        velocity.linvel.y = (up - down) * 150.;
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    asset_server: Res<AssetServer>,
    spawn_query: Query<&GridCoords, With<Spawn>>,
    player_query: Query<Entity, With<Player>>,
    assets: Res<Assets<Image>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                println!("Handle:{:?}", handle);

                let eeeeee: Handle<Image> = asset_server.load("player2.png");
                println!("Player sprite handle: {:?}", eeeeee);

                if player_query.is_empty() && handle.id() == eeeeee.id() {
                    println!("Creating players");
                    println!("{:?}", spawn_query);

                    let player = assets.get(handle).unwrap();
                    let player_height = player.texture_descriptor.size.height as f32 * 0.5;
                    let player_width = player.texture_descriptor.size.width as f32 * 0.5;

                    println!("Height: {}, Width: {}", player_height, player_width);

                    spawn_query.for_each(|cords| {
                        println!("1Spawning player at cords: x:{}, y:{}", cords.x, cords.y);
                        commands.spawn((PlayerBundle {
                            sprite_bundle: SpriteBundle {
                                transform: Transform::from_xyz(
                                    (cords.x * 16 + 8) as f32,
                                    (cords.y * 16 + 8) as f32,
                                    0.0,
                                ),
                                texture: asset_server.load("player2.png"),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            collider_bundle: ColliderBundle {
                                collider: Collider::cuboid(player_width, player_height),
                                rigid_body: RigidBody::Dynamic,
                                friction: Friction {
                                    coefficient: 0.0,
                                    combine_rule: CoefficientCombineRule::Min,
                                },
                                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                                ..Default::default()
                            },
                            weapon: Weapon{
                                ..Default::default()},
                            ..Default::default()
                        },));
                    });
                }
            }
            AssetEvent::Modified { handle: _} => {
                // an image was modified
            }
            AssetEvent::Removed { handle: _} => {
                // an image was unloaded
            }
        }
    }
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    //println!("{:?}",wall_query);

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });

                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

const ASPECT_RATIO: f32 = 16. / 9.;

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    orthographic_projection.viewport_origin = Vec2::ZERO;

                    let height = (level.px_hei as f32 / 9.).round() * 9. / 1.7;
                    let width = height * ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}
