use crate::core::world::{BlockKind, Player, PlayerControls, World};
use crate::tests::recording::{ControlsRecord, Recording, TickRecord};
use glam::DVec3;
use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::ffi::OsStr;

mod recording;

const RECORDINGS_DIR: Dir = include_dir!("recordings");

fn compare_players(sim: &Player, real: &Player) -> Result<(), String> {
    let compare_bool = |name: &str, sim: bool, real: bool| {
        if sim != real {
            return Err(format!("{} mismatch: sim={}, real={}", name, sim, real));
        }
        Ok(())
    };

    let compare_vec = |name: &str, sim: DVec3, real: DVec3| {
        if sim.distance(real) > 5e-5 {
            return Err(format!(
                "{} mismatch: sim={}, real={}, dist={}, error={}",
                name,
                sim.as_vec3(),
                real.as_vec3(),
                sim.distance(real),
                (sim-real).as_vec3(),
            ));
        }
        Ok(())
    };

    compare_vec("pos", sim.pos, real.pos)?;
    compare_vec("vel", sim.vel, real.vel)?;
    compare_bool("on_ground", sim.on_ground, real.on_ground)?;
    Ok(())
}

fn test_recording(recording: Recording, recording_name: &str) {
    if recording.ticks.is_empty() {
        panic!("Recording is empty (no ticks)!")
    }

    println!(
        "Testing recording \"{recording_name}\", length: {}, blocks: {}",
        recording.ticks.len(),
        recording.blocks_involved.len()
    );

    let mut world = World::new();
    for block_pos in recording.blocks_involved {
        world.set_block(block_pos, BlockKind::FullCube).unwrap();
    }

    let player_id = world.add_player(Player::new());

    let mut prev_controls = PlayerControls::DEFAULT;
    for i in 0..(recording.ticks.len() - 1) {
        let tick: &TickRecord = &recording.ticks[i];
        let next_tick_controls: ControlsRecord = recording.ticks[i + 1].controls;

        let sim_player = world.get_player(player_id).unwrap().clone();

        tick.player
            .apply_to(world.get_player_mut(player_id).unwrap());

        if i > 0 {
            let player = world.get_player(player_id).unwrap();
            let result = compare_players(&sim_player, player);
            if let Err(e) = result {
                eprintln!("======================================================");
                eprintln!("SIMULATION MISMATCH [\"{recording_name}\", tick={i}]");
                eprintln!("ERROR: {e}");
                eprintln!("Controls: {prev_controls:#?}");
                eprintln!("Sim player state: {sim_player:#?}");
                eprintln!("Real player state: {player:#?}");
                panic!();
            }
        }

        let player = world.get_player_mut(player_id).unwrap();
        let mut controls_map = HashMap::new();
        let controls = next_tick_controls.to_player_controls(player.angle);
        controls_map.insert(player_id, controls.clone());

        world.tick(controls_map);
        prev_controls = controls;
    }
}

#[test]
pub fn test_recordings() {
    for file in RECORDINGS_DIR.files() {
        if file.path().extension() == Some(OsStr::new("json")) {
            let recording_name = file.path().display().to_string();
            let recording: Recording = serde_json::from_slice(file.contents()).unwrap();
            test_recording(recording, &recording_name);
        }
    }
}
