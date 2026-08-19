use glam::{DVec3, IVec3};
use serde::{Deserialize, Serialize};
use crate::core::math::Angle;
use crate::core::world::{Player, PlayerControls, PlayerMoveInput};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EntityRecord {
    pub pos: DVec3,
    pub vel: DVec3,
    pub yaw: f64,
    pub pitch: f64,

    pub on_ground: bool,
    pub sprinting: bool,
    pub sneaking: bool,
}
impl EntityRecord {
    pub fn apply_to(&self, entity: &mut Player) {
        entity.pos = self.pos;
        entity.vel = self.vel;
        entity.angle = Angle::new(self.yaw, self.pitch);
        entity.on_ground = self.on_ground;
        entity.sprinting = self.sprinting;
        entity.sneaking = self.sneaking;
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ControlsRecord {
    pub move_f: bool,
    pub move_b: bool,
    pub move_r: bool,
    pub move_l: bool,

    pub jump: bool,
    pub sprint: bool,
    pub sneak: bool,
}
impl ControlsRecord {
    pub fn to_player_controls(&self, angle: Angle) -> PlayerControls {
        let mut move_forward: f64 = 0.0;
        let mut move_right: f64 = 0.0;
        if self.move_f { move_forward += 1.0; }
        if self.move_b { move_forward -= 1.0; }
        if self.move_r { move_right += 1.0; }
        if self.move_l { move_right -= 1.0; }
        PlayerControls {
            angle,
            move_input: PlayerMoveInput {
                forward: move_forward,
                right: move_right
            },
            attack: false,
            jump: self.jump,
            sprint: self.sprint,
            sneak: self.sneak,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TickRecord {
    pub player: EntityRecord,
    pub controls: ControlsRecord,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recording {
    pub ticks: Vec<TickRecord>,
    pub blocks_involved: Vec<IVec3>
}