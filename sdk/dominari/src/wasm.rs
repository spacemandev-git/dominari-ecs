use anchor_lang::prelude::Pubkey;
use solana_client_wasm::WasmClient;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::gamestate::GameState;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello Wasm!".to_string()
}

#[wasm_bindgen]
pub struct GameInstance {
    pub state: GameState
}

#[wasm_bindgen]
impl GameInstance {
    #[wasm_bindgen(constructor)]
    pub fn new(rpc: String, world: String, instance: u64) -> Self {
        GameInstance {
            state: GameState::new(
                WasmClient::new(rpc.as_str()),
                Pubkey::from_str(world.as_str()),
                instance,
            )
        }
    }
}

// Build Game State
// Update Game State on Event
// Refresh Game State
// Get Tile Info
// Attack
// Spawn
// Move
// Get Player Info
// Get Map Info