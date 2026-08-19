use glam::{Vec2, Vec3};

pub fn conv_vec3(v: Vec3) -> macroquad::prelude::Vec3 {
    macroquad::prelude::vec3(v.x, v.y, v.z)
}

pub fn conv_vec2(v: Vec2) -> macroquad::prelude::Vec2 {
    macroquad::prelude::vec2(v.x, v.y)
}