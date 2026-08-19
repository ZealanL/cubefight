use crate::core::world::{PlayerId, Player, PlayerControls, PlayerMoveInput, World};
use glam::{DVec2, DVec3};
use crate::core::math::Angle;
use crate::core::update::{collision_util, Packet};
use crate::core::update::packet::PacketData;

const GRAVITY_VEL: DVec3 = DVec3::new(0.0, -0.08, 0.0);
const GROUND_WALK_ACCEL: f64 = 0.1;
const AIR_ACCEL_SCALE: f64 = 0.2;
const SPRINT_ACCEL_SCALE: f64 = 1.3;

const JUMP_VEL_Y: f64 = 0.42;
const SPRINT_JUMP_IMPULSE_FORWARD: f64 = 0.2;
const AIR_DRAG_XZ_BASE: f64 = 0.91;
const AIR_DRAG_Y: f64 = 0.98;

fn apply_movement_speed_factors(move_input: PlayerMoveInput, is_sneaking: bool) -> PlayerMoveInput {
    const MOVEMENT_INPUT_SCALE_BASE: f64 = 0.98;
    const MOVEMENT_INPUT_SCALE_SNEAK: f64 = 0.3;

    let mut move_vec: DVec2 = move_input.into();
    if move_vec.length_squared() < 1e-7 {
        return move_input;
    }

    move_vec *= if is_sneaking {
        MOVEMENT_INPUT_SCALE_SNEAK
    } else {
        MOVEMENT_INPUT_SCALE_BASE
    };

    let speed = move_vec.length();
    if speed == 0.0 {
        return move_vec.into();
    }

    let dir = move_vec / speed;
    let dist_to_unit_square = {
        let abs_dir = dir.abs();
        let tangent = if abs_dir.y > abs_dir.x {
            abs_dir.x / abs_dir.y
        } else {
            abs_dir.y / abs_dir.x
        };
        (1.0 + (tangent * tangent)).sqrt()
    };

    let result_vec = dir * f64::min(speed * dist_to_unit_square, 1.0);
    result_vec.into()
}

fn movement_input_to_vel(move_input: PlayerMoveInput, speed: f64, angle: Angle) -> DVec3 {
    let mut world_move_input = angle.rotate_vec_yaw_only(
        DVec3::new(-move_input.right, 0.0, move_input.forward)
    );

    if world_move_input.length_squared() > 1.0 {
        world_move_input = world_move_input.normalize()
    }

    world_move_input * speed
}

pub fn make_player_move(world: &World, id: PlayerId, controls: &PlayerControls) -> Packet {
    let mut player = world.get_player(id).unwrap().clone();

    player.prev_pos = player.pos;

    // Velocity deadzone
    {
        for i in 0..3 {
            if player.vel[i].abs() < 5e-3 {
                player.vel[i] = 0.0;
            }
        }
    }

    // Update angle
    {
        player.angle = controls.angle;
    }

    // Update sneaking and sprinting
    {
        player.sneaking = controls.sneak;
        if !player.sneaking && controls.move_input.forward > 0.0 {
            player.sprinting |= controls.sprint;
        } else {
            player.sprinting = false;
        }
    }

    // Update jump
    // TODO: Add jump cooldown
    {
        if player.on_ground && controls.jump {
            player.vel.y = JUMP_VEL_Y;
            if player.sprinting {
                let (sy, cy) = player.angle.yaw.to_radians().sin_cos();
                player.vel.x += -sy * SPRINT_JUMP_IMPULSE_FORWARD;
                player.vel.z += cy * SPRINT_JUMP_IMPULSE_FORWARD;
            }
        }
    }

    // TODO: Use standing block for drag
    let drag_scale = if player.on_ground { 0.6 } else { 1.0 };

    let mut movement_speed: f64 = GROUND_WALK_ACCEL;
    if player.sprinting {
        movement_speed *= SPRINT_ACCEL_SCALE;
    }
    if !player.on_ground {
        movement_speed *= AIR_ACCEL_SCALE;
    }

    let scaled_move_input = apply_movement_speed_factors(controls.move_input, player.sneaking);
    let vel_to_add = movement_input_to_vel(scaled_move_input, movement_speed, player.angle);

    player.vel += vel_to_add;
    let collided_motion = collision_util::collide_motion(player.cur_hitbox(), player.vel, &world);
    if collided_motion.x != player.vel.x {
        player.vel.x = 0.0;
    }
    if collided_motion.z != player.vel.z {
        player.vel.z = 0.0;
    }
    let collided_vertical = collided_motion.y != player.vel.y;
    if collided_vertical {
        player.vel.y = 0.0;
    }
    player.pos += collided_motion;

    player.on_ground = collided_vertical && collided_motion.y <= 0.0;

    let drag = DVec3::new(
        AIR_DRAG_XZ_BASE * drag_scale,
        AIR_DRAG_Y,
        AIR_DRAG_XZ_BASE * drag_scale
    );

    // Prevent falling accumulation when yknow... not falling...
    if player.on_ground {
        player.vel.y = f64::max(player.vel.y, 0.0);
    }

    // Apply gravity and drag after
    player.vel += GRAVITY_VEL;
    player.vel *= drag;

    Packet::new(id, PacketData::Move {
        pos: player.pos,
        vel: player.vel,
        angle: player.angle,
        sprinting: player.sprinting,
        sneaking: player.sneaking,
        on_ground: player.on_ground,
    })
}