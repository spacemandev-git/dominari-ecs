use anchor_client::{solana_sdk::{signature::{Keypair, read_keypair_file}, signer::Signer, transaction::Transaction}};
use anchor_client::solana_client::rpc_client::RpcClient;
use dominari::{universe::Universe, world::World};

const RPC_URL:&str = "http://64.227.14.242:8899";

fn main() {
    let id01 = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap();
    let id_str = id01.to_base58_string();
    //let id02 = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id2.json")).unwrap();
    //let id2_str = id02.to_base58_string();

    let rpc_01:RpcClient = RpcClient::new(RPC_URL);

    let universe: Universe = Universe::new(RPC_URL, RPC_URL.replace("http", "wss").as_str(), Some(Keypair::from_base58_string(id_str.as_str())));
    let world: World = World::new(RPC_URL, RPC_URL.replace("http", "wss").as_str(), Some(Keypair::from_base58_string(id_str.as_str())));

    // Deploy & Register
        // Initalize World with Universe
    println!("Initalizing World....");
    let mut init_world_tx = Transaction::new_with_payer(
        world.initialize(universe.program.id().to_string().as_str(), id01.pubkey()).unwrap().as_slice(),
        Some(&id01.pubkey()),
    );
    init_world_tx.sign(&[&id01], rpc_01.get_latest_blockhash().unwrap());
    rpc_01.send_and_confirm_transaction(&init_world_tx).unwrap();
    println!("Initialized World!")
}



/*
## Scripts
    -> Deploy & Register
        - Deploy Universe, World, Systems
        -> Initalize World with Universe
        -> Register Components to Dominari World
        -> Deploy Dominari Systems
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