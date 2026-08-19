use std::f64::consts::PI;
use crate::core::world::Player;
use crate::mq_conv;
use glam::{DMat3, DVec3};
use macroquad::color;
use macroquad::color::{Color};
use macroquad::models::draw_affine_parallelepiped;

fn draw_cube(center: DVec3, size: DVec3, pivot: DVec3, rot: DMat3, color: Color) {
    let offset_from_pivot = center - pivot;
    let rotated_offset = rot * offset_from_pivot;
    let rotated_center = pivot + rotated_offset;

    let axis_x = rot.col(0).normalize();
    let axis_y = rot.col(1).normalize();
    let axis_z = rot.col(2).normalize();

    let half_x = axis_x * (size.x / 2.0);
    let half_y = axis_y * (size.y / 2.0);
    let half_z = axis_z * (size.z / 2.0);

    let origin = rotated_center - half_x - half_y - half_z;

    let edge_x = axis_x * size.x;
    let edge_y = axis_y * size.y;
    let edge_z = axis_z * size.z;

    draw_affine_parallelepiped(
        mq_conv::conv_vec3(origin.as_vec3()),
        mq_conv::conv_vec3(edge_x.as_vec3()),
        mq_conv::conv_vec3(edge_y.as_vec3()),
        mq_conv::conv_vec3(edge_z.as_vec3()),
        None,
        color
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BodyBoxType {
    Torso, Head, Arm, Leg
}

struct BodyBox {
    pub body_box_type: BodyBoxType,
    pub center_pos: DVec3,
    pub size: DVec3,
    pub pivot_y: Option<f64>,
}
impl BodyBox {
    pub const fn new(body_box_type: BodyBoxType, center_pos: DVec3, size: DVec3, pivot_y: Option<f64>) -> Self {
        Self { body_box_type, center_pos, size, pivot_y }
    }
}

pub fn draw_player_mesh(player: &Player, cur_time: f64, tick_frac: f64) {
    let draw_color = if player.hurt_timer > 0 {
        color::RED
    } else {
        color::SKYBLUE
    };

    const BODY_BOXES: [BodyBox; 6] = [
        BodyBox::new(
            BodyBoxType::Torso,
            DVec3::new(0.0, 18.0, 0.0),
            DVec3::new(8.0, 12.0,4.0),
            None
        ),
        BodyBox::new(
            BodyBoxType::Head,
            DVec3::new(0.0, 28.0, 0.0),
            DVec3::new(8.0, 8.0, 8.0),
            Some(24.0)
        ),
        BodyBox::new(
            BodyBoxType::Arm,
            DVec3::new(-5.5, 18.0, 0.0),
            DVec3::new(3.0, 12.0, 4.0),
            Some(24.0)
        ),
        BodyBox::new(
            BodyBoxType::Arm,
            DVec3::new(5.5, 18.0, 0.0),
            DVec3::new(3.0, 12.0, 4.0),
            Some(24.0)
        ),
        BodyBox::new(
            BodyBoxType::Leg,
            DVec3::new(-2.0, 6.0, 0.0),
            DVec3::new(4.0, 12.0, 4.0),
            Some(12.0)
        ),
        BodyBox::new(
            BodyBoxType::Leg,
            DVec3::new(2.0, 6.0, 0.0),
            DVec3::new(4.0, 12.0, 4.0),
            Some(12.0)
        )
    ];
    const SCALING_FACTOR: f64 = 1.8 / 32.0;

    let yaw_rot = DMat3::from_rotation_y(-player.angle.yaw.to_radians());
    let pitch_rot = DMat3::from_rotation_x(-player.angle.pitch.to_radians());

    const MAX_SWING_RAD: f64 = PI / 4.0;
    const MAX_SWING_VEL: f64 = 0.3;
    let swing_frac = (player.vel.with_y(0.0).length() / MAX_SWING_VEL).min(1.0);
    let cur_swing_angle = f64::sin(cur_time * PI * 4.0) * swing_frac * MAX_SWING_RAD;
    for body_box in BODY_BOXES {
        let side_sign = body_box.center_pos.x.signum();
        let origin = yaw_rot * body_box.center_pos;
        let size = body_box.size;
        let pivot_y = body_box.pivot_y.unwrap_or(origin.y);
        let pivot_pos = origin.with_y(pivot_y);

        let player_pos = player.prev_pos.lerp(player.pos, tick_frac);

        let mut rot = yaw_rot;
        match body_box.body_box_type {
            BodyBoxType::Head => {
                rot *= pitch_rot;
            }
            BodyBoxType::Arm => {
                rot *= DMat3::from_rotation_x(cur_swing_angle * side_sign);
            },
            BodyBoxType::Leg => {
                rot *= DMat3::from_rotation_x(cur_swing_angle * -side_sign);
            }
            _ => {}
        }
        draw_cube(
            origin*SCALING_FACTOR + player_pos,
            size*SCALING_FACTOR,
            pivot_pos*SCALING_FACTOR + player_pos,
            rot,
            draw_color
        );
    }
}