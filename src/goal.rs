use crate::components::ColliderBundle;
use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;
use crate::events::RestartLevelEvent;

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<GoalBundle>("Goal")
            .add_systems(Update, handle_col);
    }
}

#[derive(Clone, Default, Copy, Eq, PartialEq, Debug, Component)]
pub struct Goal;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub goal: Goal,
}


pub fn handle_col(
    mut evts: EventReader<CollisionEvent>,
    mut restart_event: EventWriter<RestartLevelEvent>,
    player_query: Query<Entity, With<Player>>,
    goal_query: Query<Entity, With<Goal>>,
) {
    

    let player_entity = match player_query.iter().next() {
        Some(entity) => entity,
        None => {
            println!("No player entity found");
            return;
        }
    };

    let goal_entity = match goal_query.iter().next() {
        Some(entity) => entity,
        None => {
            println!("No goal entity found");
            return;
        }
    };

    for evt in evts.read() {
        if let CollisionEvent::Started(e1, e2, _) = evt {
            println!("Collision event started");
            println!("e1: {:?}, e2: {:?}", e1, e2);
            println!("player_entity: {:?}, goal_entity: {:?}", player_entity, goal_entity);
            println!("e1 == player_entity: {:?}, e2 == goal_entity: {:?}", *e1 == player_entity, *e2 == goal_entity);
            println!("e2 == player_entity: {:?}, e1 == goal_entity: {:?}", *e2 == player_entity, *e1 == goal_entity);
            if (*e1 == player_entity && *e2 == goal_entity) || (*e1 == goal_entity && *e2 == player_entity) {
                println!("Player and goal intersecting");
                restart_event.send_default();
            }
        }
    }
}