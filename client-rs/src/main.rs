use dominari::{solana_sdk::{signature::{Keypair, read_keypair_file}}};
use dominari::{universe::Universe, world::World, dominari::{Dominari}};
use futures::executor::block_on;
use solana_client_wasm::WasmClient;
use std::env;

mod register;
use register::*;

mod map;

const RPC_URL:&str = "http://64.227.14.242:8899";

pub struct Client {
    pub id01: Keypair,
    pub rpc: WasmClient,
    pub universe: Universe,
    pub world: World,
    pub dominari: Dominari
}

fn main() {
    block_on(async_main())
}

async fn async_main() {
    let client: Client = Client {
        id01: read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap(),
        rpc: WasmClient::new(RPC_URL),
        universe: Universe::new(RPC_URL),
        world: World::new(RPC_URL, dominari::world::World::get_default_program_id()),
        dominari: Dominari::new(RPC_URL),
    };

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    match args.get(1).unwrap().as_str() {
        "register" => {
            println!("Initializing Programs...");
            register(&client).await;
        },
        "map" => {
            let instance:u64 = args.get(2).unwrap().parse().unwrap();
            let max_x:usize = args.get(3).unwrap().parse().unwrap();
            let max_y:usize = args.get(4).unwrap().parse().unwrap();
            println!("Generating Map of size ({:#},{:#}) for instance {:#}", max_x, max_y, instance);

        }
        &_ => {
            println!("Command ({}) Not Supported!", args.get(1).unwrap());
        }
    }

}

pub async fn register(client: &Client) {
    // Deploy using Deploy.bash in Terminal

    // Initalize World with Universe
    init_world(&client).await;
    
    // Register Components to World
    init_components(&client).await;
    
    // Instance World
    let instance = instance_world(&client).await;
    
    // Register Dominari Systems for all Components
    register_system_for_component(&client, instance).await;

}

/*
## Scripts


    -> Setup Features, Units, Mods
        -> Register Blueprints as Accounts on DominariSystems for each Feature, Unit, Mod
        -> Register Blueprint for Starting Card

    -> Register Player
        -> Create Player Entity
        -> Init Player by giving them a starting Unit Blueprint as a card

    -> Build Phase Sim 01
        -> Two players buy and build various features on locations

    -> Phase Phase Sim 02
        -> Two players spawn units and use features while attempting to kill other player off
*/