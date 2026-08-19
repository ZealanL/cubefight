use crate::core::world::{BlockKind, Player, PlayerId, World};
use glam::{DVec3, IVec3};
use crate::core::math::DBox3;

#[derive(Debug, Copy, Clone)]
pub struct TraceBlocksHit {
    pub hit_pos: DVec3,
    pub hit_block_pos: IVec3,
    pub hit_normal: DVec3,
    pub hit_dist: f64
}

fn trace_blocks_inner<const IS_INVERSE: bool>(from: DVec3, to: DVec3, world: &World) -> Option<TraceBlocksHit> {
    // DISCLAIMER: Originally generated (in part) by an LLM, then modified

    let ray = to - from;
    let max_dist = ray.length();
    if max_dist < 1e-6 {
        return None;
    }

    let dir = ray / max_dist;
    let step = dir.signum().as_ivec3();

    let cur_delta_t = (1.0 / dir.abs()).clamp(DVec3::ZERO, DVec3::splat(f64::INFINITY));

    let mut cur_block = from.floor().as_ivec3();
    let target_block = to.floor().as_ivec3();

    let bounds = cur_block.as_dvec3() + step.max(IVec3::ZERO).as_dvec3();
    let mut cur_max_t = (bounds - from) / dir;
    cur_max_t = DVec3::select(dir.cmpeq(DVec3::ZERO), DVec3::splat(f64::INFINITY), cur_max_t);

    let mut cur_normal = IVec3::ZERO;
    let mut cur_dist = 0.0;
    while cur_dist <= max_dist {
        if let Some(block) = world.get_block(cur_block) {
            let hit = if IS_INVERSE {
                block == BlockKind::Air
            } else {
                block != BlockKind::Air
            };
            if hit {
                return Some(TraceBlocksHit {
                    hit_pos: from + (dir * cur_dist),
                    hit_block_pos: cur_block,
                    hit_normal: cur_normal.as_dvec3(),
                    hit_dist: cur_dist
                });
            }
        }

        if cur_block == target_block {
            break;
        }

        let axis = if cur_max_t.x < cur_max_t.y && cur_max_t.x < cur_max_t.z {
            0
        } else if cur_max_t.y < cur_max_t.z {
            1
        } else {
            2
        };

        cur_dist = cur_max_t[axis];
        cur_max_t[axis] += cur_delta_t[axis];
        cur_block[axis] += step[axis];
        cur_normal = IVec3::ZERO;
        cur_normal[axis] = -step[axis];
    }

    None
}

pub fn trace_blocks(from: DVec3, to: DVec3, world: &World) -> Option<TraceBlocksHit> {
    trace_blocks_inner::<false>(from, to, world)
}

pub fn trace_blocks_inverse(from: DVec3, to: DVec3, world: &World) -> Option<TraceBlocksHit> {
    trace_blocks_inner::<true>(from, to, world)
}

//////////////

#[derive(Debug, Copy, Clone)]
pub struct TracePlayerHit {
    pub hit_pos: DVec3,
    pub hit_player: PlayerId,
    pub hit_dist: f64
}

fn intersect_ray_player(
    player: &Player,
    origin: &DVec3,
    dir: &DVec3,
    inv_dir: &DVec3,
    max_dist: f64,
) -> Option<TracePlayerHit> {
    // DISCLAIMER: Originally generated (in part) by an LLM, then modified

    let hitbox = player.cur_hitbox();
    let t1 = (hitbox.min - origin) * inv_dir;
    let t2 = (hitbox.max - origin) * inv_dir;

    let t_min_vec = t1.min(t2);
    let t_max_vec = t1.max(t2);

    let t_near = t_min_vec.max_element();
    let t_far = t_max_vec.min_element();

    if t_near <= t_far && t_far >= 0.0 && t_near <= max_dist {
        let hit_dist = t_near.max(0.0);
        Some(TracePlayerHit {
            hit_pos: origin + (dir * hit_dist),
            hit_player: player.id(),
            hit_dist
        })
    } else {
        None
    }
}

pub fn trace_players(
    from: DVec3,
    to: DVec3,
    world: &World,
    skip_player: Option<PlayerId>,
) -> Option<TracePlayerHit> {

    let ray = to - from;
    let max_dist = ray.length();
    if max_dist < 1e-6 {
        return None;
    }

    let dir = ray / max_dist;

    // Anti-NAN inverse dir
    let inv_dir = DVec3::select(
        dir.cmpeq(DVec3::ZERO),
        DVec3::splat(f64::INFINITY),
        1.0 / dir,
    );

    let mut closest_hit: Option<TracePlayerHit> = None;
    let mut closest_dist = max_dist;

    for player in world.get_player_map().values() {
        if Some(player.id()) == skip_player {
            continue;
        }

        if let Some(player_hit) = intersect_ray_player(player, &from, &dir, &inv_dir, closest_dist) {
            closest_hit = Some(player_hit);
            closest_dist = player_hit.hit_dist;
        }
    }

    closest_hit
}