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

use crate::*;
 
pub async fn init_map(client: &Client, instance:u64, max_x: u8, max_y: u8) {
    println!("Initalizing {max_x} by {max_y} map...");
    let mut init_map_tx = Transaction::new_with_payer(
        client.dominari.init_map(client.id01.pubkey(), instance, max_x, max_y).as_slice(),
        Some(&client.id01.pubkey())
    );
    init_map_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&init_map_tx).await.unwrap();
    println!("Map Entity Pubkey: {:#}", client.dominari.get_instance_index(instance).await.map);
}

/*
 * Init Map & Tile should be rewritten to build from yaml file
 * 
 */


 /*
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
let rpc = RpcClient::new(RPC_URL);
rpc.send_transaction_with_config(&init_map_tx, RpcSendTransactionConfig {
    skip_preflight: true,
    ..RpcSendTransactionConfig::default()
}).unwrap();
*/