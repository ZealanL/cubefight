use crate::core::world::{BlockKind, Chunk};
use glam::{IVec3, Vec2, Vec3};
use macroquad::models::Mesh;
use macroquad::prelude::{Texture2D, Vertex};

pub fn build_chunk_meshes(chunk: &Chunk) -> Vec<Mesh> {
    let mut meshes = Vec::new();

    const STEP_SIZE: usize = 2;

    for y_start in (0..256).step_by(STEP_SIZE) {
        let mut verts: Vec<Vertex> = Vec::new();
        let mut idcs = Vec::new();

        let chunk_min_pos = chunk.pos().min_block_pos();
        for x in 0..16 {
            for z in 0..16 {
                for y in y_start..(y_start + STEP_SIZE as i32) {
                    let block_pos = chunk_min_pos + IVec3::new(x, y, z);
                    let block_kind = chunk.get_block(block_pos).unwrap_or(BlockKind::Air);
                    if block_kind == BlockKind::Air {
                        continue;
                    }

                    const FACE_DIRS: [IVec3; 6] = [
                        IVec3::new(-1, 0, 0),
                        IVec3::new(1, 0, 0),
                        IVec3::new(0, -1, 0),
                        IVec3::new(0, 1, 0),
                        IVec3::new(0, 0, -1),
                        IVec3::new(0, 0, 1),
                    ];

                    const FACE_VERTS: [[IVec3; 4]; 6] = [
                        [
                            IVec3::new(0, 0, 0),
                            IVec3::new(0, 0, 1),
                            IVec3::new(0, 1, 1),
                            IVec3::new(0, 1, 0),
                        ],
                        [
                            IVec3::new(1, 0, 1),
                            IVec3::new(1, 0, 0),
                            IVec3::new(1, 1, 0),
                            IVec3::new(1, 1, 1),
                        ],
                        [
                            IVec3::new(0, 0, 0),
                            IVec3::new(1, 0, 0),
                            IVec3::new(1, 0, 1),
                            IVec3::new(0, 0, 1),
                        ],
                        [
                            IVec3::new(0, 1, 1),
                            IVec3::new(1, 1, 1),
                            IVec3::new(1, 1, 0),
                            IVec3::new(0, 1, 0),
                        ],
                        [
                            IVec3::new(1, 0, 0),
                            IVec3::new(0, 0, 0),
                            IVec3::new(0, 1, 0),
                            IVec3::new(1, 1, 0),
                        ],
                        [
                            IVec3::new(0, 0, 1),
                            IVec3::new(1, 0, 1),
                            IVec3::new(1, 1, 1),
                            IVec3::new(0, 1, 1),
                        ],
                    ];

                    const FACE_UVS: [Vec2; 4] = [
                        Vec2::new(0.0, 0.0),
                        Vec2::new(1.0, 0.0),
                        Vec2::new(1.0, 1.0),
                        Vec2::new(0.0, 1.0),
                    ];

                    for (fi, face_dir) in FACE_DIRS.iter().enumerate() {
                        let offset_block_pos = block_pos + face_dir;
                        if chunk.get_block(offset_block_pos) == Some(BlockKind::FullCube) {
                            continue;
                        }

                        let vert_count = verts.len() as u16;
                        idcs.extend_from_slice(&[
                            vert_count + 0,
                            vert_count + 1,
                            vert_count + 2,
                            vert_count + 0,
                            vert_count + 2,
                            vert_count + 3,
                        ]);

                        let face_verts = FACE_VERTS[fi];
                        for (i, face_vert) in face_verts.iter().enumerate() {
                            let vert_pos = block_pos + face_vert;

                            // Ambient occlusion
                            let mut num_neighbors = 0;
                            for rx in -1..=1 {
                                for ry in -1..=1 {
                                    for rz in -1..=1 {
                                        let neighbor_pos = vert_pos + IVec3::new(rx, ry, rz);
                                        if !chunk.is_block_empty(neighbor_pos) {
                                            num_neighbors += 1;
                                        }
                                    }
                                }
                            }
                            let color_scale = (255 - (num_neighbors * 5)) as u8;

                            let vert_pos_f = vert_pos.as_vec3();
                            verts.push(Vertex {
                                position: macroquad::prelude::Vec3::new(
                                    vert_pos_f.x, vert_pos_f.y, vert_pos_f.z
                                ),
                                uv: macroquad::prelude::Vec2::new(FACE_UVS[i].x, FACE_UVS[i].y),
                                color: [color_scale, color_scale, color_scale, 255],
                                normal: macroquad::prelude::Vec3::new(
                                    face_dir.x as f32,
                                    face_dir.y as f32,
                                    face_dir.z as f32,
                                )
                                .extend(1.0),
                            })
                        }
                    }
                }
            }
        }
        meshes.push(Mesh {
            vertices: verts,
            indices: idcs,
            texture: None,
        });
    }

    meshes
}
