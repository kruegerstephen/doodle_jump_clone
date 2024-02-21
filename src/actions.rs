use leafwing_input_manager::Actionlike;
use bevy::reflect::Reflect;

#[derive(
    Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect
)]
pub enum PlatformerAction {
    Right,
    Left,
    Down,
    Up,
    Move,

    Horizontal,
    Falling,

    Jump,
    Heal,
    Dash,
    Pause,
    Menus,
}

