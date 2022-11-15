/*
    -> Setup Map
    -> Instance a map of a given grid size
        -> Create Empty Map Entity
        -> Initalize Map Entity & Add Compnoents
    -> Initalize X*Y Tiles
        -> Create Empty Tile Entity
        -> Initialize Tile(x,y) Entity & Add Components
*/

use dominari::solana_sdk::{transaction::Transaction, signer::Signer};
use solana_client_wasm::utils::rpc_config::RpcSendTransactionConfig;
use crate::*;
 
pub async fn init_map(client: &Client, instance:u64, max_x: u8, max_y: u8) {
    println!("Initalizing {max_x} by {max_y} map...");
    let mut init_map_tx = Transaction::new_with_payer(
        client.dominari.init_map(client.id01.pubkey(), instance, max_x, max_y).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    init_map_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    println!("{:?}", init_map_tx); 
    //client.rpc.send_and_confirm_transaction(&init_map_tx).await.unwrap();
    
     
    client.rpc.send_transaction_with_config(&init_map_tx, RpcSendTransactionConfig {
        skip_preflight: true,
        ..RpcSendTransactionConfig::default()
    }).await.unwrap();
    

    println!("Map Entity ID: {:#}", client.dominari.get_map_by_instance(instance).await.entity_id);
}

/*
 * Init Map & Tile should be rewritten to build from yaml file
 * 
 */