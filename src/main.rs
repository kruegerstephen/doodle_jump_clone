use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod coin;
mod components;
mod goal;
mod player;
mod systems;
mod events;
mod respawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(events::EventPipelinePlugin)
        .add_systems(Startup, systems::setup_camera)
        .add_systems(Startup, systems::setup_ldtk_world)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, display_events)
        .add_systems(Update, systems::ground_detection)
        .add_systems(Update, systems::update_on_ground)
        .add_systems(Update, systems::spawn_wall_collision)
        .add_systems(Update, systems::spawn_ground_sensor)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .add_plugins(coin::CoinPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(goal::GoalPlugin)
        .add_plugins(respawn::RespawnPlugin)
        .run();
}

pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        //print collision event colliders

        println!("Received collision event: {collision_event:?}");
    }

    for contact_force_event in contact_force_events.read() {
       println!("Received contact force event: {contact_force_event:?}");
       println!("Received contact force event: {:?}", contact_force_event.collider1); 
       println!("Received contact force event: {:?}", contact_force_event.collider2); 
    }
}

pub fn setup_physics(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 10.0),
        ActiveEvents::COLLISION_EVENTS,
        ContactForceEventThreshold(10.0),
    ));
}
