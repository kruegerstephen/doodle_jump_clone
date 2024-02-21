use bevy::prelude::*;
use crate::actions::PlatformerAction;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

pub struct PlayerStateMachinePlugin;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
enum Grounded {
    Left = -1,
    Right = 1,
    Idle = 0,
}

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Jump;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Falling {
    velocity: f32,
}

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Dash;

const JUMP_VELOCITY: f32 = 10.0;

impl Plugin for PlayerStateMachinePlugin {
fn build(&self, app: &mut App) {
    app
        .add_systems(Startup, init_state_machine)
        .add_systems(Startup, init_input_mananger);
}
}

fn init_input_mananger(mut commands: Commands) {
}

fn init_state_machine(mut commands: Commands) {
       commands.spawn((

        InputManagerBundle { 
            input_map: InputMap::default()
                .insert(KeyCode::Space, PlatformerAction::Jump)
                .insert(KeyCode::A, PlatformerAction::Move)
                .build(),
            ..default()
        },
        // This state machine achieves a very rigid movement system. Consider a state machine for
        // whatever parts of your player controller that involve discrete states. Like the movement
        // in Castlevania and Celeste, and the attacks in a fighting game.
        StateMachine::default()
            // Whenever the player presses jump, jump
            .trans::<Grounded, _>(
                just_pressed(PlatformerAction::Jump),
                Falling {
                    velocity: JUMP_VELOCITY,
                },
            )
            // When the player hits the ground, idle
            .trans::<Falling, _>(grounded, Grounded::Idle),
        Grounded::Idle,
    ));
}

fn grounded() -> bool {
    println!("grounded");
    true
}

