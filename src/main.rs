use std::collections::HashMap;
use std::ops::Add;
use glam::{DVec3, IVec3, Vec3};
use macroquad::camera::{set_camera, set_default_camera, Camera3D};
use macroquad::color::Color;
use macroquad::conf::Conf;
use macroquad::input::{is_key_down, mouse_delta_position, set_cursor_grab, show_mouse, KeyCode};
use macroquad::material::{gl_use_material, load_material, MaterialParams};
use macroquad::miniquad::{Comparison, CullFace, PipelineParams, ShaderSource};
use macroquad::models::draw_mesh;
use macroquad::prelude::{clear_background, get_time, load_texture, next_frame, ImageFormat};
use macroquad::texture::Texture2D;
use crate::core::math::Angle;
use crate::core::update::{client, server};
use crate::core::world::{BlockKind, Chunk, ChunkPos, Player, PlayerControls, PlayerMoveInput, World};

mod core;
mod render;

#[cfg(test)]
mod tests;
pub(crate) mod mq_conv;

fn determine_player_controls(player: &Player) -> PlayerControls {
    let mut move_input = PlayerMoveInput::ZERO;
    if is_key_down(KeyCode::W) { move_input.forward += 1.0; }
    if is_key_down(KeyCode::S) { move_input.forward -= 1.0; }
    if is_key_down(KeyCode::D) { move_input.right += 1.0; }
    if is_key_down(KeyCode::A) { move_input.right -= 1.0; }
    PlayerControls {
        angle: player.angle,
        move_input,
        jump: is_key_down(KeyCode::Space),
        sprint: is_key_down(KeyCode::LeftShift),
        sneak: is_key_down(KeyCode::LeftControl)
    }
}

fn window_conf() -> Conf {
    Conf {
        draw_call_vertex_capacity: 100_000,
        draw_call_index_capacity: 100_000,
        ..Default::default()
    }
}

#[macroquad::main("CubeFight", window_conf)]
async fn main() {
    let mut world = World::new();
    
    let mut blocks = Vec::new();
    for xi in 0..16 {
        for zi in 0..16 {
            for yi in 0..256 {
                blocks.push(
                    if fastrand::f32() < 0.3 {
                        world.set_block(IVec3::new(xi, yi, zi), BlockKind::FullCube).unwrap();
                    }
                );
            }
        }
    }
    let chunk_meshes = render::build_chunk_meshes(
        world.get_chunk(ChunkPos::new(0, 0)).unwrap()
    );
    
    let lighting_material = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("render/shaders/vert.glsl"),
            fragment: include_str!("render/shaders/frag.glsl"),
        },
        MaterialParams {
            uniforms: vec![/* TODO */],
            textures: vec!["texture".to_string()],
            pipeline_params: PipelineParams {
                cull_face: CullFace::Nothing,
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            },
            ..Default::default()
        },
    ).unwrap();

    let player_id = world.add_player(Player::new());
    {
        let player = world.get_player_mut(player_id).unwrap();
        player.prev_pos = player.pos;
        player.pos = DVec3::new(0.0, 258.0, 0.0);
    }

    let mut last_tick_time: f64 = 0.0;
    loop {
        set_cursor_grab(true);
        show_mouse(false);
        { // Rotate player with mouse input
            let mouse_delta = mouse_delta_position();
            let player = world.get_player_mut(player_id).unwrap();
            const MOUSE_SPEED: f64 = 60.0;
            player.angle.yaw += MOUSE_SPEED * -mouse_delta.x as f64;
            player.angle.pitch += MOUSE_SPEED * mouse_delta.y as f64;
            player.angle = player.angle.fixed();
        }

        let cur_time = get_time();
        if cur_time >= last_tick_time + (1.0 / 20.0) {
            let controls = determine_player_controls(world.get_player(player_id).unwrap());
            let packet = client::make_player_move(&world, player_id, &controls);
            server::tick(vec![packet], &mut world);
            last_tick_time += (1.0 / 20.0);
        }

        clear_background(Color::new(0.1, 0.2, 0.5, 1.0));

        let tick_frac = ((cur_time - last_tick_time) * 20.0).min(1.0);
        let player= world.get_player(player_id).unwrap();
        let mut camera_pos = player.prev_pos.lerp(player.pos, tick_frac).as_vec3().add(
            Vec3::new(0.0, Player::eye_height() as f32, 0.0)
        );
        let camera_angle = Angle::new(
            player.angle.yaw,
            player.angle.pitch.clamp(-90.0 + 1e-3, 90.0 - 1e-3)
        );
        let camera_forward = camera_angle.get_forward().as_vec3();
        let camera_target = camera_pos + camera_forward;
        camera_pos -= camera_forward * 1.0;
        camera_pos.y += 0.1;

        set_camera(&Camera3D {
            position: mq_conv::conv_vec3(camera_pos),
            target: mq_conv::conv_vec3(camera_target),
            up: macroquad::prelude::Vec3::Y,
            fovy: 90.0,
            ..Default::default()
        });

        gl_use_material(&lighting_material);
        for chunk_mesh in chunk_meshes.iter() {
            draw_mesh(chunk_mesh);
        }
        for player in world.get_player_map().values() {
            render::draw_player_mesh(player, cur_time, tick_frac);
        }

        set_default_camera();
        next_frame().await
    }
}
