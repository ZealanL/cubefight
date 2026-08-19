use std::range::Range;
use glam::DVec3;
use crate::core::math::{Angle, DBox3};

pub type EntityId = u32;


#[derive(Debug, Clone)]
pub struct Player {
    id: EntityId,

    pub pos: DVec3,
    pub prev_pos: DVec3,
    pub vel: DVec3,
    pub angle: Angle,
    pub health: f64,
    pub on_ground: bool,
    pub sprinting: bool,
    pub sneaking: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: fastrand::u32(0..EntityId::MAX),

            pos: DVec3::ZERO,
            prev_pos: DVec3::ZERO,
            vel: DVec3::ZERO,
            angle: Angle::ZERO,
            health: Self::max_health(),
            on_ground: false,
            sprinting: false,
            sneaking: false,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn cur_hitbox(&self) -> DBox3 {
        Self::base_hitbox().offset(self.pos)
    }

    pub fn base_hitbox() -> DBox3 {
        DBox3::new(
            DVec3::new(-0.3, 0.0, -0.3),
            DVec3::new( 0.3, 1.8,  0.3),
        )
    }

    pub fn max_health() -> f64 {
        20.0
    }

    pub fn eye_height() -> f64 {
        1.62
    }
}