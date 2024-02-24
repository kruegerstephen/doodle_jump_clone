
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::{
    prelude::*
};
use crate::actions::PlatformerAction;
use crate::coin::Wallet;
use crate::components::ColliderBundle;
use crate::components::GroundDetection;
use bevy::time::Stopwatch;
use leafwing_input_manager::prelude::*;
use crate::player::PlayerState;


#[derive(Component, Default)]
pub struct JumpState {
    pub time: Stopwatch,
    pub rising: bool,
    pub peak_reached: bool,
}

#[derive(Clone, Default, Copy, Eq, PartialEq, Debug, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub wallet: Wallet,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle()]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    pub controller: KinematicCharacterController,
    pub state: PlayerState,
    pub jump_duration: JumpState,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[bundle()]
    pub player_input: PlayerInput,
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




