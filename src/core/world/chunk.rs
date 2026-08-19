use glam::IVec3;
use crate::core::world::BlockKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub cx: i32,
    pub cz: i32,
}
impl ChunkPos {
    pub fn new(cx: i32, cz: i32) -> Self {
        Self { cx, cz }
    }

    pub fn from_block_pos(block_pos: IVec3) -> Self{
        Self {
            cx: block_pos.x.div_euclid(16),
            cz: block_pos.z.div_euclid(16),
        }
    }

    pub fn min_block_pos(&self) -> IVec3 {
        IVec3::new(
            self.cx * 16,
            0,
            self.cz * 16,
        )
    }

    pub fn max_block_pos_excl(&self) -> IVec3 {
        IVec3::new(
            self.cx * 16 + 16,
            256,
            self.cz * 16 + 16,
        )
    }
}

#[derive(Debug)]
pub enum BlockAccessError {
    OutOfBounds
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pos: ChunkPos,
    blocks_flat: Vec<BlockKind>,
    min_pos_cache: IVec3,
    max_pos_excl_cache: IVec3,
}

impl Chunk {
    pub const TOTAL_BLOCKS: usize = 16 * 16 * 256;

    fn map_block_idx(chunk_rel_pos: IVec3) -> usize {
        let xi = chunk_rel_pos.x;
        let yi = chunk_rel_pos.y;
        let zi = chunk_rel_pos.z;
        (xi + (zi * 16) + (yi * 256)) as usize
    }

    pub fn new(pos: ChunkPos, blocks_flat: Vec<BlockKind>) -> Self {
        assert_eq!(blocks_flat.len(), Self::TOTAL_BLOCKS);
        Self {
            pos,
            blocks_flat,
            min_pos_cache: pos.min_block_pos(),
            max_pos_excl_cache: pos.max_block_pos_excl(),
        }
    }

    pub fn new_empty(pos: ChunkPos) -> Self {
        Self {
            pos,
            blocks_flat: vec![BlockKind::Air; Self::TOTAL_BLOCKS],
            min_pos_cache: pos.min_block_pos(),
            max_pos_excl_cache: pos.max_block_pos_excl(),
        }
    }

    pub fn pos(&self) -> ChunkPos {
        self.pos
    }

    pub fn contains(&self, world_block_pos: IVec3) -> bool {
        world_block_pos.cmpge(self.min_pos_cache).all()
            && world_block_pos.cmplt(self.max_pos_excl_cache).all()
    }

    pub fn get_block(&self, world_block_pos: IVec3) -> Option<BlockKind> {
        if self.contains(world_block_pos) {
            Some(self.blocks_flat[Self::map_block_idx(world_block_pos - self.min_pos_cache)])
        } else {
            None
        }
    }

    pub fn is_block_empty(&self, world_block_pos: IVec3) -> bool {
        let block = self.get_block(world_block_pos);
        block.is_none() || block.unwrap() == BlockKind::Air
    }

    pub fn set_block(&mut self, world_block_pos: IVec3, block_kind: BlockKind) -> Result<(), BlockAccessError> {
        if self.contains(world_block_pos) {
            self.blocks_flat[Self::map_block_idx(world_block_pos - self.min_pos_cache)] = block_kind;
            Ok(())
        } else {
            Err(BlockAccessError::OutOfBounds)
        }
    }

    pub fn all_blocks(&self) -> &[BlockKind] {
        &self.blocks_flat
    }
}

