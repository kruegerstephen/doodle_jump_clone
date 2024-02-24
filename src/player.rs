use crate::actions::PlatformerAction;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use statig::{
    prelude::*, InitializedStatemachine, StateOrSuperstate,

};
use crate::player_components::*;
use std::time::Duration;



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
                fall,
                jump,
                machine_events,
            ),
        )
        .register_ldtk_entity::<PlayerBundle>("Player");
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
            if time.elapsed().as_millis() % 1000 == 0 {
                println!("state: {:?}", state_machine.0.state());
            }
            match state_machine.0.state() {
                State::Idle {} => {
                    if action_state.just_pressed(
                        PlatformerAction::Jump,
                    ) {
                        println!("Jumping");
                        state_machine.0.handle(
                            &Event::Jump {
                                event_time: time.elapsed(),
                                rising: true,
                                peak_reached: false,
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
                State::Falling {} => {

                }
            }
        }
    }
}

fn fall(
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
    mut controller_outputs: Query<(Entity, &KinematicCharacterControllerOutput)>,
    mut state_machine: Query<&mut PlayerState>,
) {
    for (_, output) in &mut controller_outputs {
        println!("output: {:?}", output.grounded);
    }

    for mut state in state_machine.iter_mut() {
        match state.0.state() {
            State::Falling {} => {
                    for mut controller in controllers.iter_mut() {
                        println!("controller: {:?}", controller.translation);
                        controller.translation = match controller.translation {
                            Some(mut v) => {
                                v.y = -10.0;
                                Some(v)
                            }
                            None => Some(Vec2::new(0.0, -10.0)),
                        };
                    }
                for (_, output) in controller_outputs.iter() {
                    println!("output: {:?}", output.grounded);
                    if output.grounded {
                        state.0.handle(&Event::Land);
                    } else {

                    }
                }
            }
            _ => {}
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
            println!("transitioning to falling");
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


//Responsible for moving character
fn change_character_position(
    input: Res<Input<KeyCode>>,
    query_action_state: Query<&ActionState<PlatformerAction>>,
    mut character_controllers: Query<&mut KinematicCharacterController, With<Player>>,
    mut state: Query<&mut PlayerState>,
    player_query: Query<&mut Transform,  With<Player>>,
    mut jump_duration: Query<&mut JumpState, With<Player>>,
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
                    let jump_height = 10.; 
                    println!("jump high: {}", jump_height);
                    println!("elapsed: {}", jump.time.elapsed_secs());
                    println!("distance: {}", distance);
                    height = jump_height;
                }
                State::Falling {} => {
                    height = char_pos.y - 0.1;
                    jump.time.reset();
                }
            }
        }

            character_controller.translation = Some(Vec2::new(distance, height));
        }
    }
}


#[derive(Default)]
struct PlayerStateMachine {
    last_jump: Option<Duration>,
}
#[derive(Debug)]
pub enum Event {
    Jump { event_time: Duration, rising: bool, peak_reached: bool},
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
            Event::Jump { event_time, peak_reached, rising} => {
                self.last_jump = Some(*event_time);
                Transition(State::jumping())
            }
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn jumping(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time, peak_reached, rising} => {
                Transition(State::jumping())
            }
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn falling(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time, peak_reached, rising} => {
                Transition(State::jumping())
            }
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),

        }
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
