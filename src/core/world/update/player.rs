use crate::core::world::update::{collision_util, player_move};
use crate::core::world::{EntityId, Player, PlayerControls, World};
use glam::DVec3;

const GRAVITY_VEL: DVec3 = DVec3::new(0.0, -0.08, 0.0);
const GROUND_WALK_ACCEL: f64 = 0.1;
const AIR_ACCEL_SCALE: f64 = 0.2;
const SPRINT_ACCEL_SCALE: f64 = 1.3;

const JUMP_VEL_Y: f64 = 0.42;
const SPRINT_JUMP_IMPULSE_FORWARD: f64 = 0.2;
const AIR_DRAG_XZ_BASE: f64 = 0.91;
const AIR_DRAG_Y: f64 = 0.98;

impl World {
    pub fn tick_player_clone(&self, id: EntityId, controls: &PlayerControls) -> Player {
        let mut player = self.get_player(id).unwrap().clone();

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

        let scaled_move_input = player_move::apply_movement_speed_factors(controls.move_input, player.sneaking);
        let vel_to_add = player_move::movement_input_to_vel(scaled_move_input, movement_speed, player.angle);

        player.vel += vel_to_add;
        let collided_motion = collision_util::collide_motion(player.cur_hitbox(), player.vel, &self);
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

        player
    }
}