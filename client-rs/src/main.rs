use borsh::BorshSerialize;
use dominari::{solana_sdk::signature::{Keypair, read_keypair_file}, dominari::*, universe::SerializedComponent};
use dominari::{universe::Universe, world::World, dominari::Dominari};
use map::init_map;
use solana_client_wasm::{solana_sdk::{signer::Signer, transaction::Transaction}, WasmClient};
use std::env;
use std::fs;

mod register;
use register::*;

mod map;

pub const RPC_URL:&str = "http://64.227.14.242:8899";

pub struct Client {
    pub id01: Keypair,
    pub rpc: WasmClient,
    pub universe: Universe,
    pub world: World,
    pub dominari: Dominari
}

#[tokio::main]
async fn main() {
    let client: Client = Client {
        id01: read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap(),
        rpc: WasmClient::new(RPC_URL),
        universe: Universe::new(RPC_URL),
        world: World::new(RPC_URL, dominari::world::World::get_default_program_id()),
        dominari: Dominari::new(RPC_URL, dominari::world::World::get_default_program_id()),
    };

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    match args.get(1).unwrap().as_str() {
        "initialize" => {
            println!("Initializing Programs...");
            initialize(&client).await;
        },
        "instance" => {
            println!("Instancing a new world...");
            let i = instance(&client).await;
            println!("New Instance ID: {}", i);
        },
        "map" => {
            let instance:u64 = args.get(2).unwrap().parse().unwrap();
            let max_x:u8 = args.get(3).unwrap().parse().unwrap();
            let max_y:u8 = args.get(4).unwrap().parse().unwrap();
            println!("Generating Map of size ({:#},{:#}) for instance {:#}", max_x, max_y, instance);
            map(&client, instance, max_x, max_y).await;
        },
        "blueprints" => {
            let path = args.get(2).unwrap();
            println!("Registering {} blueprint(s) in folder {}", fs::read_dir(path).unwrap().count(), path);
            register_blueprints(&client, path).await;
        },
        &_ => {
            println!("Command ({}) Not Supported!", args.get(1).unwrap());
        }
    }

}

pub async fn initialize(client: &Client) {
    // Deploy using Deploy.bash in Terminal

    // Initalize World with Universe
    init_world(&client).await;
    
    // Register Components to World
    init_components(&client).await;

    // Register Action Bundle
    init_dominari_action_bundle(client).await;
}

pub async fn instance(client: &Client) -> u64 {
    // Instance World
    let instance = instance_world(&client).await;
    
    // Register Dominari Systems for all Components
    register_system_for_component(&client, instance).await;

    return instance;
}

pub async fn register_blueprints(client: &Client, dir: &String) {
    let paths = fs::read_dir(dir).unwrap();
    let schemas = ComponentSchema::new(&client.world.pubkey);
    for path in paths.into_iter() {

        println!("Registering {}", &path.as_ref().unwrap().path().display());
        let pathspec = path.as_ref().unwrap().path().display().to_string().replace(".toml", "").to_string();
        let name = pathspec.split("/").collect::<Vec<&str>>().pop().unwrap();
        println!("Name: {}", name);

        let blueprint: BlueprintConfig = toml::from_str(fs::read_to_string(&path.unwrap().path()).unwrap().as_str()).unwrap();
        let mut components: Vec<SerializedComponent> = vec![];
        
        if blueprint.mapmeta.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentMapMeta::get_max_size(), 
                data:  blueprint.mapmeta.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.location.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentLocation::get_max_size(), 
                data:  blueprint.location.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.feature.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentFeature::get_max_size(), 
                data:  blueprint.feature.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.owner.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentOwner::get_max_size(), 
                data:  blueprint.owner.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.value.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentValue::get_max_size(), 
                data:  blueprint.value.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.occupant.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentOccupant::get_max_size(), 
                data:  blueprint.occupant.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.player_stats.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentPlayerStats::get_max_size(), 
                data:  blueprint.player_stats.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.last_used.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentLastUsed::get_max_size(), 
                data:  blueprint.last_used.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.rank.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentRank::get_max_size(), 
                data:  blueprint.rank.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.range.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentRange::get_max_size(), 
                data:  blueprint.range.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.drop_table.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentDropTable::get_max_size(), 
                data:  blueprint.drop_table.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.uses.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentUses::get_max_size(), 
                data:  blueprint.uses.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.healing_power.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentHealingPower::get_max_size(), 
                data:  blueprint.healing_power.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.health.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentHealth::get_max_size(), 
                data:  blueprint.health.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.damage.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentDamage::get_max_size(), 
                data:  blueprint.damage.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.troop_class.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentTroopClass::get_max_size(), 
                data:  blueprint.troop_class.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.active.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"metadata".to_string()).clone(),
                max_size: ComponentActive::get_max_size(), 
                data:  blueprint.active.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        println!("Component {:?}", components);

        // Register Blueprint Tx
        let mut register_blueprint_tx = Transaction::new_with_payer(
            client.dominari.register_blueprint(client.id01.pubkey(), name.to_string(), components).await.as_slice(),
            Some(&client.id01.pubkey())
        ); 
        register_blueprint_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
        let txid = client.rpc.send_and_confirm_transaction(&register_blueprint_tx).await.unwrap();
        println!("Registered blueprint: {}", txid);
    }

}

pub async fn map(client: &Client, instance:u64, max_x:u8, max_y:u8) {
    // Initalize the Map
    init_map(client, instance, max_x, max_y).await;

    // Initalize the Tiles
    
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