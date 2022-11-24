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

}


pub use ecs::state::SerializedComponent;