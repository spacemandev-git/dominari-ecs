use anchor_lang::prelude::*;
use solana_client_wasm::WasmClient;


pub struct Dominari {
    pub client: WasmClient,
}

impl Dominari {
    pub fn new(rpc: &str) -> Self {
        return Dominari {
            client: WasmClient::new(rpc)
        }
    }

/*     pub fn get_system_signer(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"System_Signer"], &self.program.id()).0
    } */

}