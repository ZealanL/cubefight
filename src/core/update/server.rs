use glam::DVec3;
use crate::core::math::Angle;
use crate::core::update::{Packet, PacketData};
use crate::core::world::{PlayerId, World};

fn on_player_attack(from: PlayerId, to: PlayerId, world: &mut World) {
    unimplemented!();
}

fn handle_packet(sender_id: PlayerId, data: PacketData, world: &mut World) {
    match data {
        PacketData::Move {
           pos,
           vel,
           angle,
           sprinting,
           sneaking,
           on_ground,
        } => {
            let player = world.get_player_mut(sender_id).unwrap();
            player.pos = pos;
            player.vel = vel;
            player.angle = angle;
            player.sprinting = sprinting;
            player.sneaking = sneaking;
            player.on_ground = on_ground;
        }
        PacketData::Attack(target_id) => {
            on_player_attack(sender_id, target_id, world);
        }
    }
}

pub fn tick(packets: Vec<Packet>, world: &mut World) {
    let player_ids = world.get_player_map().keys().cloned().collect::<Vec<_>>();
    for player_id in player_ids {
        let player = world.get_player_mut(player_id).unwrap();
        player.prev_pos = player.pos;
    }

    for packet in packets {
        handle_packet(packet.sender_id, packet.data, world);
    }
}