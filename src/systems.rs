use crate::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use libm::{self, Libm};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::graphics::*;

use crate::resources::MouseLoc;
use crate::game::player::components::Player;
use crate::game::enemies::components::Zombie;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn((camera, MainCamera));

    let ldtk_handle = asset_server.load("test.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
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

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.level.px_hei as f32,
                ),
            };

            for player_transform in &player_query {
                //dbg!(player_transform);
                if (player_transform.translation.x < level_bounds.max.x
                    && player_transform.translation.x > level_bounds.min.x
                    && player_transform.translation.y < level_bounds.max.y
                    && player_transform.translation.y > level_bounds.min.y)
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    println!("Changing level");
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}

// pub fn update_level_selection(
//     level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
//     player_query: Query<&Transform, With<Player>>,
//     mut level_selection: ResMut<LevelSelection>,
//     ldtk_levels: Res<Assets<LdtkLevel>>,
// ) {
//     for (level_handle, level_transform) in &level_query {
//         if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
//             let level_bounds = Rect {
//                 min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
//                 max: Vec2::new(
//                     level_transform.translation.x + ldtk_level.level.px_wid as f32,
//                     level_transform.translation.y + ldtk_level.level.px_hei as f32,
//                 ),
//             };

//             for player_transform in &player_query {
//                 if player_transform.translation.x < level_bounds.max.x
//                     && player_transform.translation.x > level_bounds.min.x
//                     && player_transform.translation.y < level_bounds.max.y
//                     && player_transform.translation.y > level_bounds.min.y
//                     && !level_selection.is_match(LevelIndices.level, &ldtk_level.level)
//                 {
//                     println!("Creating new level");
//                     *level_selection = LevelSelection::Uid(ldtk_level.level.uid.clone());
//                 }
//             }
//         }
//     }
// }

pub fn blow_up_granade(
    time: Res<Time>,
    mut grenades: Query<(&mut DetonationTimer, &Transform, Entity), With<Grenade>>,
    mut commands: Commands,
    mut zombies: Query<(&Transform, &mut Health), With<Zombie>>,
    asset_server: Res<AssetServer>,
) {
    for (mut det_timer, grenade_transform, entity) in grenades.iter_mut() {
        det_timer.detonation_timer.tick(time.delta());
        if det_timer.detonation_timer.finished() {
            commands.entity(entity).despawn();
            let explosion = commands.spawn(SpriteBundle{
                texture: asset_server.load("grenade_explosion.png"),
                ..Default::default()
            }).id();
            

            commands.entity(explosion).despawn();

            for (zombie_trans, mut zombie_health) in zombies.iter_mut() {
                if zombie_trans
                    .translation
                    .distance(grenade_transform.translation)
                    < 50.0
                {
                    zombie_health.health_points -= 10;
                }
            }
        }
    }
}

pub fn bullet_collisions(
    mut bullet_collisions: EventReader<CollisionEvent>,
    mut zombie_query: Query<(&mut Health, Entity, &mut Velocity, &Transform), With<Zombie>>,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    mut commands: Commands,
) {
    for bullet in bullet_collisions.iter() {
        println!("Received collision event: {:?}", bullet.to_owned());
        let b = bullet.to_owned();
        match b {
            CollisionEvent::Started(e1, e2, _) => {
                for (bullet_transform, bullet_entity) in bullet_query.iter() {
                    for (mut health, zombie_entity, mut zombie_vel, zombie_transform) in zombie_query.iter_mut()
                    {
                        if zombie_entity.eq(&e1) || zombie_entity.eq(&e2) {
                            health.health_points -= 1;
                            println!(
                                "Entity: {:?} took 1 dmg and now has: {:?}",
                                commands.entity(zombie_entity).id(),
                                health.health_points
                            );

                            zombie_vel.linvel += (bullet_transform.translation
                                + zombie_transform.translation)
                                .truncate()
                                .normalize()
                                * 500.0;
                        } 
                    }
                    if bullet_entity.eq(&e2) {
                        commands.entity(e2).despawn_recursive();
                    } else if bullet_entity.eq(&e1) {
                        commands.entity(e1).despawn_recursive();
                    }
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _) => {}
        }
    }
}

pub fn spawn_buddy(
    mut commands: Commands,
    mouse_pos: Res<MouseLoc>,
    input: Res<Input<KeyCode>>,
    characters: Res<CharacterSheet>,
) {
    if input.just_pressed(KeyCode::B) {
        spawn_player_sprite(
            &mut commands,
            &characters,
            Vec3::new(mouse_pos.loc.x.clone(), mouse_pos.loc.y.clone(), 0.0),
        );
    }
}

#[derive(Resource)]
pub struct Edges {
    pub edges: Vec<Edge>,
}

#[derive(Resource)]
pub struct Polygons {
    pub visibility_points: Vec<(f32, f32, f32)>,
}

#[derive(Default, Debug)]
pub struct Edge {
    pub sx: f32,
    pub sy: f32,
    pub ex: f32,
    pub ey: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    edge_id: [u32; 4],
    edge_exist: [bool; 4],
}

#[derive(Resource)]
pub struct Cells {
    pub cells: Vec<Vec<Cell>>,
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
    asset_server: Res<AssetServer>,
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

    const NORTH: usize = 0;
    const SOUTH: usize = 1;
    const EAST: usize = 2;
    const WEST: usize = 3;

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

                //Edges for shadows
                let mut vec_edges: Vec<Edge> = vec![];
                vec_edges.clear();
                let mut cells = vec![
                    vec![
                        Cell {
                            edge_id: [0, 0, 0, 0],
                            edge_exist: [false, false, false, false]
                        };
                        (width + 1).try_into().unwrap()
                    ];
                    (height + 1).try_into().unwrap()
                ];
                println!("Width: {}, Height: {}", width, height);
                for y in 0..height {
                    for x in 0..width + 1 {
                        let i = &GridCoords { x: x, y: y };
                        let n = &GridCoords { x: x, y: y - 1 };
                        let s = &GridCoords { x: x, y: y + 1 };
                        let e = &GridCoords { x: x + 1, y: y };
                        let w = &GridCoords { x: x - 1, y: y };

                        match level_walls.contains(i) {
                            true => {
                                let yi: usize = y.try_into().unwrap_or_default();
                                let xi: usize = x.try_into().unwrap_or_default();
                                if !level_walls.contains(w) {
                                    let ix: usize = (x).try_into().unwrap();
                                    let iy: usize = (y - 1).try_into().unwrap_or_default();
                                    if cells[iy][ix].edge_exist[WEST] {
                                        let edgei: usize =
                                            cells[iy][ix].edge_id[WEST].try_into().unwrap();
                                        vec_edges[edgei].ey += 16.0;
                                        cells[yi][xi].edge_id[WEST] = cells[iy][ix].edge_id[WEST];
                                        cells[yi][xi].edge_exist[WEST] = true;
                                    } else {
                                        let edge = Edge {
                                            sx: (x as f32) * 16.0,
                                            sy: y as f32 * 16.0,
                                            ex: x as f32 * 16.0,
                                            ey: (y as f32) * 16.0 + 16.0,
                                        };
                                        let edge_id = vec_edges.len();
                                        vec_edges.push(edge);

                                        cells[yi][xi].edge_id[WEST] = edge_id as u32;
                                        cells[yi][xi].edge_exist[WEST] = true;
                                    }
                                }
                                if !level_walls.contains(e) {
                                    let ix: usize = (x).try_into().unwrap();
                                    let iy: usize = (y - 1).try_into().unwrap_or_default();
                                    if cells[iy][ix].edge_exist[EAST] {
                                        let edgeid: usize =
                                            cells[iy][ix].edge_id[EAST].try_into().unwrap();
                                        vec_edges[edgeid].ey += 16.0;
                                        cells[yi][xi].edge_id[EAST] = cells[iy][ix].edge_id[EAST];
                                        cells[yi][xi].edge_exist[EAST] = true;
                                    } else {
                                        let edge = Edge {
                                            sx: (x as f32 + 1.0) * 16.0,
                                            sy: y as f32 * 16.0,
                                            ex: (x as f32 + 1.0) * 16.0,
                                            ey: y as f32 * 16.0 + 16.0,
                                        };
                                        let edge_id = vec_edges.len();
                                        vec_edges.push(edge);

                                        cells[yi][xi].edge_id[EAST] = edge_id as u32;
                                        cells[yi][xi].edge_exist[EAST] = true;
                                    }
                                }
                                if !level_walls.contains(n) {
                                    let ix: usize = (x - 1).try_into().unwrap_or_default();
                                    let iy: usize = (y).try_into().unwrap();
                                    if cells[iy][ix].edge_exist[NORTH] {
                                        let edgei: usize =
                                            cells[iy][ix].edge_id[NORTH].try_into().unwrap();
                                        vec_edges[edgei].ex += 16.0;
                                        cells[yi][xi].edge_id[NORTH] = cells[iy][ix].edge_id[NORTH];
                                        cells[yi][xi].edge_exist[NORTH] = true;
                                    } else {
                                        let edge = Edge {
                                            sx: (x as f32) * 16.0,
                                            sy: (y as f32) * 16.0,
                                            ex: x as f32 * 16.0 + 16.0,
                                            ey: (y as f32) * 16.0,
                                        };
                                        let edge_id = vec_edges.len();
                                        vec_edges.push(edge);

                                        cells[yi][xi].edge_id[NORTH] = edge_id as u32;
                                        cells[yi][xi].edge_exist[NORTH] = true;
                                    }
                                }
                                if !level_walls.contains(s) {
                                    let ix: usize = (x - 1).try_into().unwrap_or_default();
                                    let iy: usize = (y).try_into().unwrap();
                                    if cells[iy][ix].edge_exist[SOUTH] {
                                        let edgei: usize =
                                            cells[iy][ix].edge_id[SOUTH].try_into().unwrap();
                                        vec_edges[edgei].ex += 16.0;
                                        cells[yi][xi].edge_id[SOUTH] = cells[iy][ix].edge_id[SOUTH];
                                        cells[yi][xi].edge_exist[SOUTH] = true;
                                    } else {
                                        let edge = Edge {
                                            sx: x as f32 * 16.0,
                                            sy: (y as f32 + 1.0) * 16.0,
                                            ex: x as f32 * 16.0 + 16.0,
                                            ey: (y as f32 + 1.0) * 16.0,
                                        };
                                        let edge_id = vec_edges.len();
                                        vec_edges.push(edge);

                                        cells[yi][xi].edge_id[SOUTH] = edge_id as u32;
                                        cells[yi][xi].edge_exist[SOUTH] = true;
                                    }
                                }
                            }
                            false => {}
                        }
                    }
                }
                vec_edges.iter().for_each(|edge| {
                    commands.spawn(SpriteBundle {
                        texture: asset_server.load("point_end.png"),
                        transform: Transform {
                            translation: Vec3::from_array([edge.ex, edge.ey, 0.0]),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    commands.spawn(SpriteBundle {
                        texture: asset_server.load("point_start.png"),
                        transform: Transform {
                            translation: Vec3::from_array([edge.sx, edge.sy, 1.0]),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

                commands.insert_resource(Edges { edges: vec_edges });
                println!("Added edges ");

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
                            .insert(GlobalTransform::default())
                            .insert(ShadowCaster);
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
    mouse_pos: Res<MouseLoc>,
    player_pos: Query<&Transform, With<Player>>,
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

                    if let Ok(player_position) = player_pos.get_single() {
                        let camera_pos_offset =
                            (mouse_pos.loc - player_position.translation.truncate()).normalize();
                        let distance = mouse_pos
                            .loc
                            .distance(player_position.translation.truncate());

                        camera_transform.translation.x =
                            (player_translation.x - level_transform.translation.x - width / 2.
                                + camera_pos_offset.x * distance / 5.0)
                                .clamp(0., level.px_wid as f32 - width);
                        camera_transform.translation.y =
                            (player_translation.y - level_transform.translation.y - height / 2.
                                + camera_pos_offset.y * distance / 5.0)
                                .clamp(0., level.px_hei as f32 - height);

                        camera_transform.translation.x += level_transform.translation.x;
                        camera_transform.translation.y += level_transform.translation.y;
                    }
                }
            }
        }
    }
}
