use glam::{DVec2, DVec3};
use serde::Serialize;
use crate::core::math::Angle;
use crate::core::world::PlayerMoveInput;


pub fn apply_movement_speed_factors(move_input: PlayerMoveInput, is_sneaking: bool) -> PlayerMoveInput {
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

pub fn movement_input_to_vel(move_input: PlayerMoveInput, speed: f64, angle: Angle) -> DVec3 {
    let mut world_move_input = angle.rotate_vec_yaw_only(
        DVec3::new(-move_input.right, 0.0, move_input.forward)
    );

    if world_move_input.length_squared() > 1.0 {
        world_move_input = world_move_input.normalize()
    }

    world_move_input * speed
}

