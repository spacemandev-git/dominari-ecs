pub mod universe;
pub mod world;
pub mod dominari;
pub mod util;
pub mod gamestate;

// Export Solana Client so no need to reimport it
pub use solana_client_wasm::solana_sdk;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello World!".to_string()
}