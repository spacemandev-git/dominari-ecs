use anchor_lang::prelude::Pubkey;
use solana_client_wasm::WasmClient;

pub struct Universe {
    pub client: WasmClient
}

impl Universe {
    pub fn new(rpc: &str) -> Self {
        return Universe {
            client: WasmClient::new(rpc)
        }
    }

    pub fn get_keys_from_id(world_instance: Pubkey, ids: Vec<u64>) -> Vec<Pubkey> {
        let mut keys = vec![];
        for id in ids {
            keys.push(Pubkey::find_program_address(&[
                b"Entity",
                id.to_be_bytes().as_ref(),
                world_instance.to_bytes().as_ref()
            ], &ecs::id()).0);
        }
        keys
    }

    pub fn get_world_instance(world: Pubkey, instance:u64) -> Pubkey {
        Pubkey::find_program_address(&[
            b"World",
            world.to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0 
    }
}


pub use ecs::state::SerializedComponent;