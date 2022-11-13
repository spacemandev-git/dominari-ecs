/*
    -> Deploy & Register
        - Deploy Universe, World, Systems
        -> Initalize World with Universe
        -> Register Components to Dominari World
        -> Instance a World
        -> Register DominariSystems for Each of the Registered Components
*/

use anchor_client::{solana_sdk::{signer::Signer, transaction::Transaction}};

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


/*
pub fn init_components(client: &Client) -> RelevantComponentKeys {
    println!("Current components registered: {:#}", client.world.get_world_config().1.components);
    let schemas = ComponentSchema::get_schemas(&client.world);
    for schema in schemas.iter() {
        println!("Registering Component: {}", schema.url);
        let mut comp_tx = Transaction::new_with_payer(
            client.world.register_component(schema.url.clone(), client.id01.pubkey()).unwrap().as_slice(),
            Some(&client.id01.pubkey())
        );
        comp_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
        client.rpc.send_and_confirm_transaction(&comp_tx).unwrap();
    }
    println!("Components after registration loop: {:#}", client.world.get_world_config().1.components);
    return RelevantComponentKeys { 
        metadata: schemas.get(0).unwrap().key,
        mapmeta: schemas.get(1).unwrap().key, 
        location: schemas.get(2).unwrap().key, 
        feature: schemas.get(3).unwrap().key, 
        owner: schemas.get(4).unwrap().key, 
        value: schemas.get(5).unwrap().key, 
        occupant: schemas.get(6).unwrap().key, 
        player_stats: schemas.get(7).unwrap().key, 
        last_used: schemas.get(8).unwrap().key, 
        rank: schemas.get(9).unwrap().key, 
        range: schemas.get(10).unwrap().key, 
        drop_table: schemas.get(11).unwrap().key, 
        uses: schemas.get(12).unwrap().key, 
        healing_power: schemas.get(13).unwrap().key, 
        health: schemas.get(14).unwrap().key, 
        damage: schemas.get(15).unwrap().key, 
        troop_class: schemas.get(16).unwrap().key, 
        active: schemas.get(17).unwrap().key
    }
}

pub fn instance_world(client: &Client) -> u64 {
    println!("Current Instances: {:#}", client.world.get_world_config().1.instances);
    println!("Registering new instance...");
    let mut new_instance_tx = Transaction::new_with_payer(
        client.world.instance_world(client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey())
    );
    new_instance_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&new_instance_tx).unwrap();
    let instance = client.world.get_world_config().1.instances;
    println!("Instance registered: {:#}", instance);
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
        client.world.add_components_to_system_registration(ComponentSchema::get_schemas(&client.world), client.dominari.get_system_signer(), instance, client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey())
    );
    add_comp_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&add_comp_tx).unwrap();
    println!("Dominari registered for all components!", );
}
*/