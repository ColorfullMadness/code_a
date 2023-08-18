use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}   

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Spawn;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SpawnBundle {
    spawn: Spawn,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    //#[sprite_bundle("player.png")]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    //#[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub weapon: Weapon, 
    //#[worldly]
    pub worldly: Worldly,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    //#[from_entity_instance]
    //items: Items,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    //#[from_entity_instance]
    pub entity_instance: EntityInstance,
}

#[derive(Clone, Default, Component)]
pub struct Weapon {
    pub fire_rate: FireRate,
    pub ammo: Ammo,
    pub reload_timer: ReloadTimer,
}

#[derive(Clone, Component, Debug)]
pub struct FireRate{
    pub timer: Timer
}

impl Default for FireRate {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(0.1, TimerMode::Repeating) 
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct Ammo{
    pub bullets: u32,
}

impl Default for Ammo {
    fn default() -> Self {
        Self {
            bullets: 30,
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct ReloadTimer{
    pub reload_timer: Timer,
}

impl Default for ReloadTimer {
    fn default() -> Self {
        Self { reload_timer: Timer::from_seconds(2.0, TimerMode::Once) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Zombie;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ZombieBundle {
    #[sprite_bundle("zombie.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    pub zombie: Zombie,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub health: Health,
}

#[derive(Clone, Component, Debug, Eq, PartialEq)]
pub struct Health {
    pub health_points: i32, 
}

impl Default for Health{
    fn default() -> Self {
        Self {
            health_points: 2,
        }
    }
}

// impl From<&EntityInstance> for Health {
//     fn from(entity_instance: &EntityInstance) -> Self {
//         Health(
//             entity_instance
//                 .get_int_field("health")
//                 .expect("health field should be correctly typed")
//                 .to_owned()
//         )
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct ShadowCaster;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Bullet;

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct Target {pub target: Vec2}

#[derive(Clone, Default, Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub collider_bundle: ColliderBundle,

    pub bullet: Bullet,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Grenade;

#[derive(Clone, Default, Bundle)]
pub struct GrenadeBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub collider_bundle: ColliderBundle, 

    pub timer: DetonationTimer, 
    pub grenade: Grenade,
}

#[derive(Clone, Component, Debug, Default)]
pub struct DetonationTimer{
    pub detonation_timer: Timer,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(3., 6.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            "Zombie" => ColliderBundle {
                collider: Collider::ball(12.0),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}