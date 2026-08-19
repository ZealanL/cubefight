mod player;
mod player_move;
mod collision_util;

use std::collections::HashMap;
use crate::core::world::{EntityId, World};
use crate::core::world::player_controls::PlayerControls;

impl World {
    /// NOTE: Players without controls will not be updated
    pub fn tick(&mut self, player_controls_set: HashMap<EntityId, PlayerControls>) {
        for &player_id in player_controls_set.keys() {
            let controls = &player_controls_set[&player_id];
            let ticked_player = self.tick_player_clone(player_id, controls);
            self.players.insert(player_id, ticked_player);
        }
    }
}