use anchor_client::{solana_sdk::{signature::{Keypair, read_keypair_file}}};
use anchor_client::solana_client::rpc_client::RpcClient;
use dominari::{universe::Universe, world::{World}, dominari::Dominari};

mod register;
use register::*;

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
    register(&client);

}

pub fn register(client: &Client) {
    // Deploy using Deploy.bash in Terminal

    // Initalize World with Universe
    init_world(&client);
    
    // Register Components to World
    init_components(&client);
    
    // Instance World
    let instance = instance_world(&client);
    
    // Register Dominari Systems for all Components
    register_system_for_component(&client, instance)

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

    -> Phase Phase Sim 02
        -> Two players spawn units and use features while attempting to kill other player off
*/