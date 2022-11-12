use anchor_client::{solana_sdk::{signer::Signer, transaction::Transaction}};
use dominari::{world::{ComponentSchema}};

use crate::*;

pub fn init_world(client: &Client) {
    println!("Initalizing World....");
    let mut init_world_tx = Transaction::new_with_payer(
        client.world.initialize(client.universe.program.id().to_string().as_str(), client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey()),
    );
    init_world_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&init_world_tx).unwrap();
    println!("Initialized World!")
}

pub fn init_components(client: &Client) {
    println!("Current components registered: {:#}", client.world.get_world_config().components);
    for schema in ComponentSchema::get_schemas() {
        println!("Registering Component: {}", schema.url);
        let mut comp_tx = Transaction::new_with_payer(
            client.world.register_component(schema.url, client.id01.pubkey()).unwrap().as_slice(),
            Some(&client.id01.pubkey())
        );
        comp_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
        client.rpc.send_and_confirm_transaction(&comp_tx).unwrap();
    }
    println!("Components after registration loop: {:#}", client.world.get_world_config().components);
}

pub fn instance_world(client: &Client) -> u64 {
    println!("Current Instances: {:#}", client.world.get_world_config().instances);
    println!("Registering new instance...");
    let mut new_instance_tx = Transaction::new_with_payer(
        client.world.instance_world(client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey())
    );
    new_instance_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&new_instance_tx).unwrap();
    let instance = client.world.get_world_config().instances;
    println!("Instances after registration: {:#}", instance);
    return instance;
}

pub fn register_system_for_component(client: &Client, instance:u64) {
    // Register System Tx
    println!("Registering Dominari System...");
    let mut system_register_tx = Transaction::new_with_payer(
        client.world.register_system(client.dominari.get_system_signer(), instance, client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey())
    );
    system_register_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&system_register_tx).unwrap();

    // Register Components for System Tx
    println!("Adding components to Dominari registration...", );
    let mut add_comp_tx = Transaction::new_with_payer(
        client.world.add_components_to_system_registration(ComponentSchema::get_schemas(), client.dominari.get_system_signer(), instance, client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey())
    );
    add_comp_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&add_comp_tx).unwrap();
    println!("Dominari registered for all components!", );
}