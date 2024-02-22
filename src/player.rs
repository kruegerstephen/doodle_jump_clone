use crate::actions::PlatformerAction;
use crate::coin::Wallet;
use crate::components::ColliderBundle;
use crate::components::GroundDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use statig::{
    prelude::*, InitializedStatemachine, StateOrSuperstate,

};

use std::char;
use std::time::Duration;
use bevy::time::Stopwatch;


#[derive(Component, Default)]
pub struct JumpDuration {
    time: Stopwatch
}

const MOVEMENT_SPEED: f32 = 1.;

/// Plugin for spawning the player and controlling them.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                camera_fit_inside_current_level,
                change_character_position,
                modify_character_controller,
                fall,
                machine_events,
            ),
        )
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}


#[derive(Clone, Default, Copy, Eq, PartialEq, Debug, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub wallet: Wallet,
    pub controller: KinematicCharacterController,
    pub player_input: PlayerInput,
    pub state: PlayerState,
    pub jump_duration: JumpDuration,
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


#[derive(Bundle)]
pub struct PlayerInput {
    input: InputManagerBundle<PlatformerAction>
}
impl Default for PlayerInput {
    fn default() -> Self {
        use PlatformerAction::*;

        let mut input_map = InputMap::default();

        // basic movement
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(
            SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.1),
            Horizontal,
        );

        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Jump
        input_map.insert(KeyCode::Space, PlatformerAction::Jump);
        input_map.insert(GamepadButtonType::South, PlatformerAction::Jump);

        input_map.insert(KeyCode::E, PlatformerAction::Dash);
        input_map.insert(GamepadButtonType::RightTrigger2, PlatformerAction::Dash);

        input_map.insert(KeyCode::Return, PlatformerAction::Pause);
        input_map.insert(GamepadButtonType::Start, PlatformerAction::Pause);

        input_map.insert(KeyCode::I, PlatformerAction::Menus);
        input_map.insert(GamepadButtonType::Select, PlatformerAction::Menus);
        input_map.set_gamepad(Gamepad { id: 0 });

        Self {
            input: InputManagerBundle::<PlatformerAction> {
                input_map,
                ..Default::default()
            },
        }
    }
}




fn machine_events(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
    mut commands: Commands,
    mut controllers: Query<(
        &KinematicCharacterControllerOutput,
        &mut PlayerState,
    )>,
    time: Res<Time>,
) {
    for (_output, mut state_machine) in
        &mut controllers
    {
        match state_machine.0.state() {
            // State::Idle {  } => todo!(),
            State::Jumping {} => {
                if let Some(last_jump) =
                    state_machine.0.last_jump
                {
                    println!("Jump state");
                    if (time.elapsed() - last_jump)
                        > Duration::from_millis(500)
                    {
                        state_machine
                            .0
                            .handle(&Event::Fall);
                    }
                }
            }
            // State::Crouching {  } => todo!(),
            // State::Falling {  } => todo!(),
            // State::Healing {  } => todo!(),
            _ => {}
        }
    }
    for action_state in &query_action_state {
        for (output, mut state_machine) in
            &mut controllers
        {
            match state_machine.0.state() {
                State::Idle {} => {
                    if action_state.just_pressed(
                        PlatformerAction::Jump,
                    ) {
                        state_machine.0.handle(
                            &Event::Jump {
                                event_time: time.elapsed(),
                            },
                        );
                    }
                }
                State::Jumping {} => {
                     info!("jumping");
                    if let Some(last_jump) =
                        state_machine.0.last_jump
                    {
                        if output.grounded
                            && 
                            // systems can run fast enough that the newly jumping
                            // player can still be in their original pre-takeoff contact
                            // with the ground
                            //
                            // maybe replace with "last_left_ground" field?
                            time.elapsed() - last_jump
                                > Duration::from_millis(50)
                        {
                            state_machine
                                .0
                                .handle(&Event::Land);
                        }
                    }
                }
                State::Crouching {} => {
                    // info!("crouching");
                }
                State::Healing {} => {
                    // info!("healing");
                }
                State::Falling {} => {
                    info!("falling");
                }
            }
        }
    }
}

fn fall(
    mut commands: Commands,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &Velocity,
        &mut PlayerState,
        &ActionState<PlatformerAction>,
    )>,
    time: Res<Time>,
) {
    for (
        mut controller,
        output,
        velocity,
        mut state_machine,
        action_state,
    ) in &mut controllers
    {
        if let State::Falling {} = state_machine.0.state() {
            if output.grounded {
                state_machine.0.handle(&Event::Land);
            } else {
                println!("falling handler");
                controller.translation =
                    match controller.translation {
                        Some(mut v) => {
                            v.y = -2.0;
                            Some(v)
                        }
                        None => Some(Vec2::new(0.0, -2.0)),
                    };
            }
        }
    }
}
fn jump(
    mut commands: Commands,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        // &KinematicCharacterControllerOutput,
        &Velocity,
        &mut PlayerState,
        &ActionState<PlatformerAction>,
    )>,
    time: Res<Time>,
) {
    for (
        mut controller,
        // output,
        velocity,
        mut state_machine,
        action_state,
    ) in &mut controllers
    {
        if action_state
            .just_released(PlatformerAction::Jump)
        {
            state_machine.0.handle(&Event::Fall);
        } else if let State::Jumping {} =
            state_machine.0.state()
        {
                println!("jumping handler");
            controller.translation =
                match controller.translation {
                    Some(mut v) => {
                        v.y = 10.0;
                        Some(v)
                    }
                    None => Some(Vec2::new(0.0, 10.0)),
                };
        }
    }
}

#[derive(Default)]
struct PlayerStateMachine {
    last_jump: Option<Duration>,
}
#[derive(Debug)]
pub enum Event {
    Jump { event_time: Duration },
    Heal,
    Crouch,
    Land,
    Fall,
}

#[derive(Component)]
pub struct PlayerState(
    InitializedStatemachine<PlayerStateMachine>,
);
impl Default for PlayerState {
    fn default() -> Self {
        Self(
            PlayerStateMachine::default()
                .state_machine()
                .init(),
        )
    }
}


#[state_machine(
    initial = "State::idle()",
    on_dispatch = "Self::on_dispatch",
    on_transition = "Self::on_transition",
    state(derive(Debug)),
    superstate(derive(Debug))
)]
impl PlayerStateMachine {
    fn on_transition(
        &mut self,
        source: &State,
        target: &State,
    ) {
        info!(
            "transitioned from `{:?}` to `{:?}`",
            source, target
        );
    }

    fn on_dispatch(
        &mut self,
        state: StateOrSuperstate<PlayerStateMachine>,
        event: &Event,
    ) {
        info!(
            "dispatched `{:?}` to `{:?}`",
            event, state
        );
    }
    #[state]
    fn idle(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                self.last_jump = Some(*event_time);
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn jumping(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn healing(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn crouching(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn falling(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
}

//Responsible for moving character
fn change_character_position(
    input: Res<Input<KeyCode>>,
    query_action_state: Query<&ActionState<PlatformerAction>>,
    mut character_controllers: Query<&mut KinematicCharacterController, With<Player>>,
    mut state: Query<&mut PlayerState>,
    player_query: Query<&mut Transform,  With<Player>>,
    mut jump_duration: Query<&mut JumpDuration, With<Player>>,
    time: Res<Time>, 
) {
    if let Ok(Transform {
        ..
    }) = player_query.get_single()
    {
        
        let mut jump = jump_duration.single_mut();

        for query in query_action_state.iter() {
            if query.just_pressed(PlatformerAction::Jump) {
            }
            if query.just_released(PlatformerAction::Left) {
                info!("Left");
            }
            if query.just_released(PlatformerAction::Move) {
                info!("Move");
            }
            if query.just_released(PlatformerAction::Right) {
                info!("Right");
            }
            if query.just_released(PlatformerAction::Up) {
                info!("Up");
            }
        }



        for mut character_controller in character_controllers.iter_mut() {
            let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
            let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
            let distance = (right - left) * MOVEMENT_SPEED;
            let mut height = 0.; 
            let char_pos = character_controller.translation.unwrap_or_default();
//           character_controller.translation = Some(Vec2::new(distance, -1.));

 //           if input.just_pressed(KeyCode::Space) {
                //ENTER JUMP STATE
                //IF IN JUMP STATE, LERP TOWARDS JUMP HEIGHT instead of just moving there instantly
                //EXIT JUMP STATE WHEN GROUNDED
//                character_controller.translation = Some(Vec2::new(0., JUMP_HEIGHT));
  //          }
            //

        for state in state.iter_mut() {
            match state.0.state() {
                State::Idle {} => {
                    height = char_pos.y - 0.1;
                }
                State::Jumping {} => {
                    jump.time.tick(time.delta());
                    let jump_height = lerp(char_pos.y, 2., jump.time.elapsed_secs() / 0.3);
                    println!("jump high: {}", jump_height);
                    println!("elapsed: {}", jump.time.elapsed_secs());
                    println!("distance: {}", distance);
                    height = jump_height;
                }
                State::Crouching {} => {}
                State::Falling {} => {
                    height = char_pos.y - 0.1;
                    jump.time.reset();
                }
                State::Healing {} => {}
            }
        }

            character_controller.translation = Some(Vec2::new(distance, height));
        }
    }
}


fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
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
