use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use crate::coin::Wallet;

use std::collections::HashSet;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
    pub active_events: ActiveEvents,
    pub controller: KinematicCharacterController,
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub rotation_constraints: LockedAxes,
    pub active_events: ActiveEvents,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}


#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}
#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        //MOVE TO Plugins??  
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(6., 14.),
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                controller: KinematicCharacterController{
                    offset: CharacterLength::Absolute(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },
            "Goal" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                gravity_scale: GravityScale(1.),
                active_events: ActiveEvents::COLLISION_EVENTS,
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(0.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}



impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        SensorBundle::default()
    }
}


