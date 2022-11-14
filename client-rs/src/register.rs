/*
    -> Deploy & Register
        - Deploy Universe, World, Systems
        -> Initalize World with Universe
        -> Register Components to Dominari World
        -> Instance a World
        -> Register DominariSystems for Each of the Registered Components
*/

use anchor_client::{solana_sdk::{signer::Signer, transaction::Transaction, instruction::Instruction}};
use dominari::dominari::ComponentSchema;

use crate::*;

pub fn init_world(client: &Client) {
    println!("Initalizing World....");
    let mut init_world_tx = Transaction::new_with_payer(
        client.world.initialize(client.id01.pubkey()).as_slice(),
        Some(&client.id01.pubkey()),
    );
    init_world_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&init_world_tx).unwrap();
    println!("Initialized World!")
}

pub async fn init_components(client: &Client) {
    println!("Current components registered: {:#}", client.world.get_world_config().await.1.components);
    let mut comp_ixs: Vec<Instruction> = vec![];
    for schema in ComponentSchema::get_all_schema_urls().iter() {
        let mut ix = client.world.register_component(schema, client.id01.pubkey());
        comp_ixs.append(&mut ix);
    }
    for comp_ix in comp_ixs.iter() {
        let ix = comp_ix.clone();
        let mut tx = Transaction::new_with_payer(
            [ix].as_slice(),
            Some(&client.id01.pubkey())
        );
        tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
        let sig = client.rpc.send_and_confirm_transaction(&tx).unwrap().to_string();    
        println!("Component Registered: {sig}");
    }
    println!("Components after registration loop: {:#}", client.world.get_world_config().await.1.components);
}

pub async fn instance_world(client: &Client) -> u64 {
    println!("Current Instances: {:#}", client.world.get_world_config().await.1.instances);
    println!("Registering new instance...");
    let mut new_instance_tx = Transaction::new_with_payer(
        client.world.instance_world(client.id01.pubkey()).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    new_instance_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&new_instance_tx).unwrap();
    let instance = client.world.get_world_config().await.1.instances;
    println!("Instance registered: {:#}", instance);
    return instance;
}

pub async fn register_system_for_component(client: &Client, instance:u64) {
    // Register System Tx
    println!("Registering Dominari System...");
    let mut system_register_tx = Transaction::new_with_payer(
        client.world.register_system(client.dominari.get_system_signer(), instance, client.id01.pubkey()).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    system_register_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&system_register_tx).unwrap();

    // Register Components for System Tx
    println!("Adding components to Dominari registration...", );
    let mut add_comp_tx = Transaction::new_with_payer(
        client.world.add_components_to_system_registration(ComponentSchema::new(&client.world).get_all_component_keys(), client.dominari.get_system_signer(), instance, client.id01.pubkey()).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    add_comp_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&add_comp_tx).unwrap();
    println!("Dominari registered for all components!", );
}
