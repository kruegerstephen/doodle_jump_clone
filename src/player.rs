use crate::coin::Wallet;
use crate::components::ColliderBundle;
use crate::components::GroundDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

//Convert to seldom_state
enum player_state {
    Idle,
    Walking,
    Jumping,
    Falling
}

const MOVEMENT_SPEED: f32 = 1.;
const JUMP_HEIGHT: f32 = 100.;

/// Plugin for spawning the player and controlling them.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_fit_inside_current_level,change_character_position, modify_character_controller))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Clone, Default, Copy, Eq, PartialEq, Debug, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    pub wallet: Wallet,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,

}



//Responsible for moving character
fn change_character_position(
    input: Res<Input<KeyCode>>,
    mut character_controllers: Query<&mut KinematicCharacterController, With<Player>>
) {
    for mut character_controller in character_controllers.iter_mut() {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
        let distance = (right - left) * MOVEMENT_SPEED;
        character_controller.translation = Some(Vec2::new(distance, -1.));


        if input.just_pressed(KeyCode::Space) {
            //ENTER JUMP STATE 
            //IF IN JUMP STATE, LERP TOWARDS JUMP HEIGHT instead of just moving there instantly 
            //EXIT JUMP STATE WHEN GROUNDED
            character_controller.translation = Some(Vec2::new(0., JUMP_HEIGHT));
        }
    }
}


/* Read the character controller collisions stored in the character controller’s output. */
fn modify_character_controller(
    mut controllers: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    for (entity, output) in &controllers {
//        info!("Entity: {:?}, Output: {:?}", entity, output);
    }
}





//This shit is copied straight from the ldtk example. It's in here because it tracks the player,
//could probably be moved to a separate camera plugin. 
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
    level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_iid) in &level_query {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            if level_selection.is_match(&LevelIndices::default(), level) {
                let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                orthographic_projection.viewport_origin = Vec2::ZERO;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let width = (level.px_wid as f32 / 16.).round() * 16.;
                    let height = width / ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}
