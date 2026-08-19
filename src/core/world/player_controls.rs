use glam::{DVec2, DVec3};
use crate::core::math::Angle;

#[derive(Debug, Copy, Clone)]
pub struct PlayerMoveInput {
    pub forward: f64,
    pub right: f64
}
impl PlayerMoveInput {
    pub const ZERO: Self = Self { forward: 0.0, right: 0.0 };
}

impl From<PlayerMoveInput> for DVec2 {
    fn from(move_input: PlayerMoveInput) -> Self {
        Self::new(move_input.forward, move_input.right)
    }
}

impl From<DVec2> for PlayerMoveInput {
    fn from(move_input: DVec2) -> Self {
        Self {
            forward: move_input.x,
            right: move_input.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerControls {
    pub angle: Angle,
    pub move_input: PlayerMoveInput,
    pub attack: bool,
    pub jump: bool,
    pub sprint: bool,
    pub sneak: bool,
}
impl PlayerControls {
    pub const DEFAULT: Self = Self {
        angle: Angle::ZERO,
        move_input: PlayerMoveInput::ZERO,
        attack: false,
        jump: false,
        sprint: false,
        sneak: false,
    };
}