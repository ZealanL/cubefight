use glam::{DVec3, IVec3};
use crate::core::math::{DBox3, PosExt};
use crate::core::world::{BlockKind, World};

fn calc_axis_offset(hitbox: &DBox3, other: &DBox3, mut offset: f64, axis: usize) -> f64 {
    let (a, b) = match axis {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    };

    let overlaps = other.max[a] > hitbox.min[a] && other.min[a] < hitbox.max[a]
        && other.max[b] > hitbox.min[b] && other.min[b] < hitbox.max[b];
    if !overlaps {
        return offset;
    }

    if offset > 0.0 && other.max[axis] <= hitbox.min[axis] {
        let d = hitbox.min[axis] - other.max[axis];
        if d < offset { offset = d; }
    } else if offset < 0.0 && other.min[axis] >= hitbox.max[axis] {
        let d = hitbox.max[axis] - other.min[axis];
        if d > offset { offset = d; }
    }
    offset
}

fn resolve_axes(bb: &mut DBox3, colliders: &[DBox3], mut delta: DVec3) -> DVec3 {
    for c in colliders { delta.y = calc_axis_offset(c, bb, delta.y, 1); }
    *bb = bb.offset(DVec3::new(0.0, delta.y, 0.0));

    for c in colliders { delta.x = calc_axis_offset(c, bb, delta.x, 0); }
    *bb = bb.offset(DVec3::new(delta.x, 0.0, 0.0));

    for c in colliders { delta.z = calc_axis_offset(c, bb, delta.z, 2); }
    *bb = bb.offset(DVec3::new(0.0, 0.0, delta.z));

    delta
}

pub fn collide_motion(hitbox: DBox3, motion: DVec3, world: &World) -> DVec3 {
    let from_hitbox = hitbox;
    let to_hitbox = from_hitbox.offset(motion);
    let min_block_pos = to_hitbox.min.to_block_pos();
    let max_block_pos = to_hitbox.max.to_block_pos();

    let mut to_resolve = Vec::new();
    for x in min_block_pos.x..=max_block_pos.x {
        for y in min_block_pos.y..=max_block_pos.y {
            for z in min_block_pos.z..=max_block_pos.z {
                let block_pos = IVec3::new(x, y, z);
                let block_full_hitbox = DBox3::from_block_pos(block_pos);
                let Some(block) = world.get_block(block_pos) else {
                    continue;
                };

                match block {
                    BlockKind::Air => continue,
                    BlockKind::FullCube => (),
                    _ => unimplemented!()
                }

                to_resolve.push(block_full_hitbox);
            }
        }
    }

    let mut resolved_hitbox = from_hitbox.clone();
    let new_motion = resolve_axes(&mut resolved_hitbox, &to_resolve, motion);
    new_motion
}