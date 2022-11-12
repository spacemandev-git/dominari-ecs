use anchor_client::{solana_sdk::{signature::{Keypair, read_keypair_file}, signer::Signer, transaction::Transaction}};
use anchor_client::solana_client::rpc_client::RpcClient;
use dominari::{universe::Universe, world::{World, ComponentSchema}, dominari::Dominari};

const RPC_URL:&str = "http://64.227.14.242:8899";

pub struct Client {
    pub id01: Keypair,
    pub rpc: RpcClient,
    pub universe: Universe,
    pub world: World,
    pub dominari: Dominari
}

fn main() {
    let client: Client = Client {
        id01: read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap(),
        rpc: RpcClient::new(RPC_URL),
        universe: Universe::new(RPC_URL, RPC_URL.replace("http", "wss").as_str(), None),
        world: World::new(RPC_URL, RPC_URL.replace("http", "wss").as_str(), None),
        dominari: Dominari::new(RPC_URL, RPC_URL.replace("http", "wss").as_str(), None),
    };

    /* Deploy & Register */
    // Deploy using Deploy.bash in Terminal
    // Initalize World with Universe
    init_world(&client);
    // Register Components to World
    init_components(&client);
    // Instance World
    let instance = instance_world(&client);
    // Register Dominari Systems for all Components
    register_system_for_component(&client, instance)
    // Register Systems for Each Component

}

fn init_world(client: &Client) {
    println!("Initalizing World....");
    let mut init_world_tx = Transaction::new_with_payer(
        client.world.initialize(client.universe.program.id().to_string().as_str(), client.id01.pubkey()).unwrap().as_slice(),
        Some(&client.id01.pubkey()),
    );
    init_world_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
    client.rpc.send_and_confirm_transaction(&init_world_tx).unwrap();
    println!("Initialized World!")
}

fn init_components(client: &Client) {
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

fn instance_world(client: &Client) -> u64 {
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

fn register_system_for_component(client: &Client, instance:u64) {
    for schema in ComponentSchema::get_schemas() {
        println!("Registering Dominari System for Component ({})", schema.url);
        let mut system_tx = Transaction::new_with_payer(
            client.world.register_system_for_component(schema.url, client.dominari.get_system_signer(), instance, client.id01.pubkey()).unwrap().as_slice(),
            Some(&client.id01.pubkey())
        );
        system_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().unwrap());
        client.rpc.send_and_confirm_transaction(&system_tx).unwrap();
    }
    println!("Dominari registered for all components!", );
}

/*
## Scripts
    -> Deploy & Register
        - Deploy Universe, World, Systems
        -> Initalize World with Universe
        -> Register Components to Dominari World
        -> Instance a World
        -> Register DominariSystems for Each of the Registered Components

    -> Setup Map
        -> Instance a map of a given grid size
            -> Create Empty Map Entity
            -> Initalize Map Entity & Add Compnoents
        -> Initalize X*Y Tiles
            -> Create Empty Tile Entity
            -> Initialize Tile(x,y) Entity & Add Components

    -> Setup Features, Units, Mods
        -> Register Blueprints as Accounts on DominariSystems for each Feature, Unit, Mod
        -> Register Blueprint for Starting Card

    -> Register Player
        -> Create Player Entity
        -> Init Player by giving them a starting Unit Blueprint as a card

    -> Build Phase Sim 01
        -> Two players buy and build various features on locations

    -> Phase Phase Sim 01
        -> Two players spawn units and use features while attempting to kill other player off
*/