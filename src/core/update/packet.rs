use glam::DVec3;
use crate::core::math::Angle;
use crate::core::world::{Player, PlayerId};

#[derive(Debug, Clone)]
pub enum PacketData {
    Move {
        pos: DVec3,
        vel: DVec3,
        angle: Angle,
        sprinting: bool,
        sneaking: bool,
        on_ground: bool,
    },
    Attack(PlayerId)
}

impl PacketData {
    pub fn ordering(&self) -> usize {
        match self {
            PacketData::Move { .. } => 1,
            PacketData::Attack(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub sender_id: PlayerId,
    pub data: PacketData,
}

impl Packet {
    pub fn new(sender_id: PlayerId, data: PacketData) -> Self {
        Packet { sender_id, data }
    }
}