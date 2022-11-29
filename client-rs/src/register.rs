
use solana_client_wasm::{solana_sdk::{signer::Signer, transaction::Transaction, instruction::Instruction}};
use dominari::dominari::ComponentSchema;

use crate::*;


pub async fn instance_world(client: &Client) -> u64 {
    println!("Current Instances: {:#}", client.world.get_world_config().await.1.instances);
    println!("Registering new instance...");
    let mut new_instance_tx = Transaction::new_with_payer(
        client.world.instance_world(client.id01.pubkey()).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    new_instance_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&new_instance_tx).await.unwrap();
    
    // TODO: Should be replaced with fetching instance account
    // maybe just return world instance from SDK
    let instance = client.world.get_world_config().await.1.instances;
    
    
    println!("Instance registered: {:#}", instance);
    return instance;
}

