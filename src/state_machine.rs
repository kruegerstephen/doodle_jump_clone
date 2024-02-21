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
    app.add_plugins(InputManagerPlugin::<PlatformerAction>::default())
        .add_plugins(StateMachinePlugin)
        .add_systems(Startup, init_state_machine)
        .add_systems(Startup, init_input_mananger)
        .add_systems(Update, walk)
        .add_systems(Update, fall);
}
}

fn init_input_mananger(mut commands: Commands) {
commands.spawn(
    InputManagerBundle { 
        input_map: InputMap::default()
            .insert(KeyCode::Space, PlatformerAction::Jump)
            .build(),
        ..default()
    }
);
}

fn init_state_machine(mut commands: Commands) {
       commands.spawn((
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
            .trans::<Falling, _>(grounded, Grounded::Idle)
            // When the player is grounded, set their movement direction
            .trans_builder(value_unbounded(PlatformerAction::Move), |_: &Grounded, value| {
                Some(match value {
                    value if value > 0.5 => Grounded::Right,
                    value if value < -0.5 => Grounded::Left,
                    _ => Grounded::Idle,
                })
            }),
        Grounded::Idle,
    ));
}

fn grounded(In(entity): In<Entity>, fallings: Query<(&Transform, &Falling)>) -> bool {
    let (transform, falling) = fallings.get(entity).unwrap();
    transform.translation.y <= 0. && falling.velocity <= 0.
} 


const PLAYER_SPEED: f32 = 200.;

fn walk(mut groundeds: Query<(&mut Transform, &Grounded)>, time: Res<Time>) {
    for (mut transform, grounded) in &mut groundeds {
        transform.translation.x += *grounded as i32 as f32 * time.delta_seconds() * PLAYER_SPEED;
    }
}

const GRAVITY: f32 = -1000.;

fn fall(mut fallings: Query<(&mut Transform, &mut Falling)>, time: Res<Time>) {
    for (mut transform, mut falling) in &mut fallings {
        let dt = time.delta_seconds();
        falling.velocity += dt * GRAVITY;
        transform.translation.y += dt * falling.velocity;
    }
}
