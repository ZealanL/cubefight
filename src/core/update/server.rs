use glam::DVec3;
use crate::core::math::Angle;
use crate::core::update::{Packet, PacketData};
use crate::core::world::{PlayerId, World};

fn on_player_attack(from_player: PlayerId, to_player: PlayerId, world: &mut World) {
    let (from_pos, sprint_kb) = {
        let player = world.get_player(from_player).unwrap();
        (player.pos, player.sprint_kb)
    };
    let to_player = world.get_player_mut(to_player).unwrap();

    if to_player.hurt_timer > 0 {
        return;
    }

    {
        // Yes, this is ACTUALLY what minecraft does... it's absurd.
        let mut dx: f64 = to_player.pos.x - from_pos.x;
        let mut dz: f64 = to_player.pos.z - from_pos.z;
        while (dx*dx) + (dz*dz) < 1e-4 {
            dx = (fastrand::f64() - fastrand::f64()) * 1e-2;
            dz = (fastrand::f64() - fastrand::f64()) * 1e-2;
        }

        let dist = (dx * dx + dz * dz).sqrt();

        let power_horizontal = if sprint_kb { 0.9 } else { 0.4 };

        to_player.vel /= 2.0;
        to_player.vel.x += (dx / dist) * power_horizontal;
        to_player.vel.z += (dz / dist) * power_horizontal;
        to_player.vel.y = f64::min(0.4, to_player.vel.y + 0.4);
    }

    to_player.hurt_timer = 10;
    to_player.health -= 1.0;

    world.get_player_mut(from_player).unwrap().sprint_kb = false;
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
            if sprinting && !player.sprinting {
                player.sprint_kb = true;
            } else {
                player.sprint_kb &= sprinting;
            }
            player.sprinting = sprinting;
            player.sneaking = sneaking;
            player.on_ground = on_ground;
        }
        PacketData::Attack(target_id) => {
            on_player_attack(sender_id, target_id, world);
        }
    }
}

pub fn tick(mut packets: Vec<Packet>, world: &mut World) {
    let player_ids = world.get_player_map().keys().cloned().collect::<Vec<_>>();
    for &player_id in &player_ids {
        let player = world.get_player_mut(player_id).unwrap();
        player.prev_pos = player.pos;
    }

    packets.sort_by(
        |a, b| usize::cmp(&a.data.ordering(), &b.data.ordering())
    );

    for packet in packets {
        handle_packet(packet.sender_id, packet.data, world);
    }

    for &player_id in &player_ids {
        let player = world.get_player_mut(player_id).unwrap();
        if player.hurt_timer > 0 {
            player.hurt_timer -= 1;
        }
    }
}