use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::events::RestartLevelEvent;

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, respawn_world);
    }
}

fn respawn_level(
    mut commands: Commands,
    mut restart_level_events: EventReader<RestartLevelEvent>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
) {
    for _ in restart_level_events.read() {
        let level_selection_iid = match level_selection.as_ref() {
            LevelSelection::Iid(iid) => iid,
            _ => panic!("level should always be selected by iid in this example"),
        };

        for (level_entity, level_iid) in levels.iter() {
            if level_iid == level_selection_iid {
                commands.entity(level_entity).insert(Respawn);
            }
        }
    }
}

fn respawn_world(
    mut restart_level_events: EventReader<RestartLevelEvent>,
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
) {
    for _ in restart_level_events.read() { 
        commands.entity(ldtk_projects.single()).insert(Respawn);
    }
}



