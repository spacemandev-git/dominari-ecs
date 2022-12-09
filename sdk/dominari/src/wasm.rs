use std::{str::FromStr, collections::HashMap};

use anchor_lang::prelude::Pubkey;
use ecs::account::WorldInstance;
use solana_client_wasm::WasmClient;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use crate::{dominari::Dominari, util::fetch_account};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;


#[wasm_bindgen]
pub fn greet() -> String {
    "Hello Wasm 2!".to_string()
}

#[wasm_bindgen]
pub struct GameInstance {
    #[wasm_bindgen(skip)]
    pub rpc: WasmClient,
    #[wasm_bindgen(skip)]
    pub action_bundle: Dominari,
    pub instance: u64,
}

#[wasm_bindgen]
impl GameInstance {
    #[wasm_bindgen]
    pub async fn new(rpc:String, world: String, instance:u64) -> Self {
        console_error_panic_hook::set_once();
        let client = WasmClient::new(&rpc);

        // Check if Instance is an actual game!
        let instance_pubkey = Pubkey::find_program_address(&[
            b"World",
            Pubkey::from_str(world.as_str()).unwrap().to_bytes().as_ref(),
            instance.to_be_bytes().as_slice(),
        ], &ecs::id()).0;

        // Should panic if the game instance isn't found
        let _instance_acc:WorldInstance = fetch_account(&client, &instance_pubkey).await.unwrap_throw();

        GameInstance {
            instance,
            rpc: client,
            action_bundle: Dominari {
                world: Pubkey::from_str(world.as_str()).unwrap(),
                state: HashMap::new()
            }    
        }
    }

    pub fn get_world_key(&self) -> String {
        self.action_bundle.world.to_string()
    }

    pub async fn build_game_state(&mut self) {
        self.action_bundle.build_gamestate(&self.rpc, self.instance).await;
    }

    pub async fn load_blueprints(&mut self, names: JsValue) {
        let blueprints: Vec<String> = serde_wasm_bindgen::from_value(names).unwrap();
        self.action_bundle.get_mut_gamestate(self.instance).blueprints.insert_blueprint_strings(&blueprints);
    }

    pub async fn update_entity(&mut self, entity_id: u64) {
        self.action_bundle.get_mut_gamestate(self.instance).update_entity(entity_id).await;
    }

    pub async fn refresh_state(&mut self) {
        self.action_bundle.get_mut_gamestate(self.instance).load_state().await;
    }

    pub fn debug(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.action_bundle.get_gamestate(self.instance).index.as_ref().unwrap().clone().map.to_string()).unwrap()
    }

    pub fn spawn_unit(&self, payer: String, player_id: u64, x: u8, y:u8, card: String) -> Result<JsValue, String> {
        let state = self.action_bundle.get_gamestate(self.instance);
        let tile = state.get_tile(x, y).unwrap();
        let occupant = state.get_entity_occupant(&tile.0).unwrap().occupant_id;
        // Check if Tile is EMPTY == Use SPAWN UNIT
        // If tile is OCCUPIED == USE MODIFY UNIT
    
        if occupant.is_none() {
            // SPAWN UNIT
            let compute_buget_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);
            let mut spawn_ix = self.action_bundle.spawn_unit(
                Pubkey::from_str(&payer.as_str()).unwrap(),
                state.instance,
                player_id,
                tile.0,
                state.blueprints.get_blueprint_by_name(&card).unwrap()
            );
    
            spawn_ix.insert(0, compute_buget_ix); 
    
            return Ok(serde_wasm_bindgen::to_value(spawn_ix.as_slice()).unwrap());
            //send_tx_skip_preflight(spawn_unit_tx);
        } else {
            // Check if unit belongs to the player_id
            // If it does, play the modify_unit tx
            return Err("Tile Occupied!".to_string());
        }
    
    }

    pub fn move_unit(&self, payer: String, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> JsValue {
        let state = self.action_bundle.get_gamestate(self.instance);
        let from_tile = state.get_tile(from_x, from_y).unwrap();
        let to_tile = state.get_tile(to_x, to_y).unwrap();

        let instructions = self.action_bundle.move_unit(
            Pubkey::from_str(&payer.as_str()).unwrap(),
            self.instance,
            from_tile.0,
            to_tile.0
        );

        serde_wasm_bindgen::to_value(instructions.as_slice()).unwrap()
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