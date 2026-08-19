mod player;
mod block_kind;
mod chunk;
mod player_controls;
pub mod update;

use std::collections::HashMap;
use glam::IVec3;
pub use block_kind::*;
pub use chunk::*;
pub use player::*;
pub use player_controls::*;

#[derive(Debug, Clone)]
pub struct World {
    players: HashMap<EntityId, Player>,
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            chunks: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, entity: Player) -> EntityId {
        let id = entity.id();
        let existing = self.players.insert(id, entity);
        if existing.is_some() {
            panic!("Entity already exists");
        }
        id
    }

    pub fn remove_player(&mut self, id: EntityId) -> Option<Player> {
        self.players.remove(&id)
    }

    pub fn get_player(&self, entity_id: EntityId) -> Option<&Player> {
        self.players.get(&entity_id)
    }

    pub fn get_player_mut(&mut self, entity_id: EntityId) -> Option<&mut Player> {
        self.players.get_mut(&entity_id)
    }

    pub fn get_player_map(&self) -> &HashMap<EntityId, Player> {
        &self.players
    }

    pub fn get_chunk_map(&self) -> &HashMap<ChunkPos, Chunk> {
        &self.chunks
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&chunk_pos)
    }
    
    pub fn get_block(&self, block_pos: IVec3) -> Option<BlockKind> {
        let chunk = self.chunks.get(&ChunkPos::from_block_pos(block_pos));
        if let Some(chunk) = chunk {
            chunk.get_block(block_pos)
        } else {
            None
        }
    }

    pub fn set_block(&mut self, block_pos: IVec3, kind: BlockKind) -> Result<(), BlockAccessError> {
        let chunk_pos = ChunkPos::from_block_pos(block_pos);
        if !self.chunks.contains_key(&chunk_pos) {
            self.chunks.insert(chunk_pos, Chunk::new_empty(chunk_pos));
        }

        let chunk = self.chunks.get_mut(&chunk_pos);
        if let Some(chunk) = chunk {
            chunk.set_block(block_pos, kind)
        } else {
            unreachable!()
        }
    }
}