use borsh::BorshSerialize;
use dominari::{solana_sdk::{signature::{Keypair, read_keypair_file}, instruction::Instruction}, dominari::*, universe::SerializedComponent};
use dominari::{universe::Universe, world::World, dominari::Dominari};
use serde::Deserialize;
//use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_client_wasm::{solana_sdk::{signer::Signer, transaction::Transaction}, WasmClient};
use tokio::task::JoinHandle;
use std::env;
use std::fs;
use rand::Rng;

mod state;
use crate::state::*;

mod util;
use crate::util::*;

pub const RPC_URL:&str = "http://64.227.14.242:8899";

#[tokio::main]
async fn main() {
    let mut client: Client = Client {
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
            // Deploy using Deploy.bash in Terminal

            // Initalize World with Universe
            init_world(&client).await;
            
            // Register Components to World
            init_components(&client).await;

            // Register Action Bundle
            init_dominari_action_bundle(&client).await;
        },
        "blueprints" => {
            let path = args.get(2).unwrap();
            println!("Registering {} blueprint(s) in folder {}", fs::read_dir(path).unwrap().count(), path);
            register_blueprints(&client, path).await;
        },
        "setup_game" => {
            let path = args.get(2).unwrap();
            let instance:u64;
            if args.get(3).is_some() {
                instance = args.get(3).unwrap().parse::<u64>().unwrap();
            } else {
                let mut rng = rand::thread_rng();
                instance = rng.gen();
            }
            println!("Setting up game per config at {} with instance {:#} ...", path, instance);
            setup_game(&mut client, path, instance).await;
        },
        "index" => {
            let instance = args.get(2).unwrap().parse::<u64>().unwrap();
            client.dominari.build_gamestate(instance).await;
            println!("Index {:?}", &client.dominari.get_gamestate(instance).index.as_ref());
        
        },
        "debug" => {
        }
        &_ => {
            println!("Command ({}) Not Supported!", args.get(1).unwrap());
        }
    }

}

pub async fn init_world(client: &Client) {
    println!("Initalizing World....");
    let mut init_world_tx = Transaction::new_with_payer(
        client.world.initialize(client.id01.pubkey()).as_slice(),
        Some(&client.id01.pubkey()),
    );
    init_world_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&init_world_tx).await.unwrap();
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
        tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
        send_tx_async(client.rpc.clone(), tx.clone());
        //let sig = client.rpc.send_and_confirm_transaction(&tx).await.unwrap().to_string();    
        //println!("Component Registered: {sig}");
    }
    println!("Components after registration loop: {:#}", client.world.get_world_config().await.1.components);
}

pub async fn register_system_for_component(client: &Client, instance:u64) {
    // Create System Registration for a given Instance
    println!("Registering Dominari system for instance {}...", instance);
    let mut system_register_tx = Transaction::new_with_payer(
        client.world.register_system(client.dominari.get_system_signer(), instance, client.id01.pubkey()).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    system_register_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&system_register_tx).await.unwrap();

    // Register Components for given system registration
    println!("Adding components to Dominari registration...", );
    let mut add_comp_tx = Transaction::new_with_payer(
        client.world.add_components_to_system_registration(ComponentSchema::new(&client.world.pubkey).get_all_component_keys(), client.dominari.get_system_signer(), instance, client.id01.pubkey()).await.as_slice(),
        Some(&client.id01.pubkey())
    );
    add_comp_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&add_comp_tx).await.unwrap();
    println!("Dominari registered for all components!", );
}

pub async fn init_dominari_action_bundle(client: &Client) {
    //  Register Dominari Game
    println!("Registering Dominari Action Bundle...");
    let mut init_tx = Transaction::new_with_payer(
        client.dominari.init_action_bundle(client.id01.pubkey()).as_slice(),
        Some(&client.id01.pubkey())
    );
    init_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&init_tx).await.unwrap();
    println!("Dominari action bundle registered!");
}

pub async fn register_blueprints(client: &Client, dir: &String) {
    let paths = fs::read_dir(dir).unwrap();
    let schemas = ComponentSchema::new(&client.world.pubkey);

    let mut blueprint_txs: Vec<JoinHandle<()>> = vec![];
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
                component_key: schemas.get_component_pubkey(&"location".to_string()).clone(),
                max_size: ComponentLocation::get_max_size(), 
                data:  blueprint.location.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.feature.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"feature".to_string()).clone(),
                max_size: ComponentFeature::get_max_size(), 
                data:  blueprint.feature.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.owner.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"owner".to_string()).clone(),
                max_size: ComponentOwner::get_max_size(), 
                data:  blueprint.owner.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.value.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"value".to_string()).clone(),
                max_size: ComponentValue::get_max_size(), 
                data:  blueprint.value.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.occupant.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"occupant".to_string()).clone(),
                max_size: ComponentOccupant::get_max_size(), 
                data:  blueprint.occupant.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.player_stats.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"player_stats".to_string()).clone(),
                max_size: ComponentPlayerStats::get_max_size(), 
                data:  blueprint.player_stats.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.last_used.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"last_used".to_string()).clone(),
                max_size: ComponentLastUsed::get_max_size(), 
                data:  blueprint.last_used.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.rank.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"rank".to_string()).clone(),
                max_size: ComponentRank::get_max_size(), 
                data:  blueprint.rank.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.range.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"range".to_string()).clone(),
                max_size: ComponentRange::get_max_size(), 
                data:  blueprint.range.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.drop_table.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"drop_table".to_string()).clone(),
                max_size: ComponentDropTable::get_max_size(), 
                data:  blueprint.drop_table.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.uses.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"uses".to_string()).clone(),
                max_size: ComponentUses::get_max_size(), 
                data:  blueprint.uses.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.healing_power.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"healing_power".to_string()).clone(),
                max_size: ComponentHealingPower::get_max_size(), 
                data:  blueprint.healing_power.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.health.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"health".to_string()).clone(),
                max_size: ComponentHealth::get_max_size(), 
                data:  blueprint.health.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.damage.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"damage".to_string()).clone(),
                max_size: ComponentDamage::get_max_size(), 
                data:  blueprint.damage.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.troop_class.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"troop_class".to_string()).clone(),
                max_size: ComponentTroopClass::get_max_size(), 
                data:  blueprint.troop_class.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.active.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"active".to_string()).clone(),
                max_size: ComponentActive::get_max_size(), 
                data:  blueprint.active.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.cost.is_some() {
            components.push(SerializedComponent { 
                component_key: schemas.get_component_pubkey(&"cost".to_string()).clone(),
                max_size: ComponentCost::get_max_size(), 
                data:  blueprint.cost.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        // Register Blueprint Tx
        let mut register_blueprint_tx = Transaction::new_with_payer(
            client.dominari.register_blueprint(client.id01.pubkey(), name.to_string(), components).await.as_slice(),
            Some(&client.id01.pubkey())
        ); 
        register_blueprint_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());

        blueprint_txs.push(send_tx_async(client.rpc.clone(), register_blueprint_tx.clone()));
    }

    for tx in blueprint_txs {
        tx.await.unwrap();
    }
    
}

pub async fn setup_game(client: &mut Client, path: &String, instance: u64) {
    let config:Game = toml::from_str(fs::read_to_string(path.as_str()).unwrap().as_str()).unwrap();
    //println!("Config Found: {:?}", config);

    // Instance the game (will instance the world)
    let mut create_game_tx = Transaction::new_with_payer(
        &client.dominari.init_game(client.id01.pubkey(), instance, config.config),
        Some(&client.id01.pubkey())
    );
    create_game_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    
    client.rpc.send_and_confirm_transaction(&create_game_tx).await.unwrap();
    // After the Game & World are Instanced, we need to Register Dominari for all Components
    register_system_for_component(&client, instance).await;

    // Create Map & Tiles & Features
    map(client, instance, config.map).await;
}

pub async fn map(client: &mut Client, instance:u64, map: MapConfig) {
    // Initialize the map
    let max_x = map.mapmeta.max_x;
    let max_y = map.mapmeta.max_y;
    println!("Initalizing {max_x} by {max_y} map...");
    let mut init_map_tx = Transaction::new_with_payer(
        client.dominari.init_map(client.id01.pubkey(), instance, max_x, max_y).as_slice(),
        Some(&client.id01.pubkey())
    );
    init_map_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&init_map_tx).await.unwrap();
    
    // Initalize the Tiles
    let mut tile_txs = vec![];
    for row in 0..map.mapmeta.max_x {
        for col in 0..map.mapmeta.max_y {
            let mut init_tile_tx = Transaction::new_with_payer(
                client.dominari.init_tile(client.id01.pubkey(), instance, row, col, map.cost_per_tile).as_slice(),
                Some(&client.id01.pubkey())
            );
            init_tile_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
            //client.rpc.send_and_confirm_transaction(&init_tile_tx).await.unwrap();
            tile_txs.push(send_tx_async(client.rpc.clone(), init_tile_tx.clone()));
        }
    }
    for tile_tx in tile_txs {
        tile_tx.await.unwrap();
    }

    // Reloads the state after map and tiles are initalized
    client.dominari.build_gamestate(instance).await;

    // Init the Features
    for feature in map.features {
        //let tile = client.dominari.get_gamestate(instance).get_tile(&client.dominari.get_instance_index(instance).await.tiles, feature.x, feature.y).unwrap();
        let tile = client.dominari.get_gamestate(instance).get_tile(&client.dominari.get_gamestate(instance).index.as_ref().unwrap().tiles, feature.x, feature.y).unwrap();
        //println!("Tile ({},{}) is {}", feature.x, feature.y, tile.0);

        let blueprint = Dominari::get_blueprint_key(&feature.feature);
        let mut feature_tx = Transaction::new_with_payer(&client.dominari.init_feature(client.id01.pubkey(), instance, tile.0, blueprint).as_slice(), Some(&client.id01.pubkey()));
        feature_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
        
        //let rpc:RpcClient = RpcClient::new(RPC_URL); rpc.send_transaction_with_config(&feature_tx, RpcSendTransactionConfig {skip_preflight: true, .. Default::default()}).unwrap();
        let sig = client.rpc.send_and_confirm_transaction(&feature_tx).await.unwrap();
        println!("Feature {} created at ({},{}): {}", feature.feature, feature.x, feature.y, sig);
    }   
}


/*
## Scripts
    -> Deploy & Register
        - Deploy Universe, World, Systems
        -> Initalize World with Universe
        -> Register Components to Dominari World
        -> Register DominariSystems for Each of the Registered Components

    -> Setup Features, Units, Mods
        -> Register Blueprints as Accounts on DominariSystems for each Feature, Unit, Mod

    -> Setup Game
        -> Instance a Game (will instance a world underneath)
        -> Instance a map of a given grid size
        -> Instance Tiles

    -> Register Player
        -> Create Player Entity
        -> Init Player by giving them a starting Unit Blueprint as a card

    -> Build Phase Sim 01
        -> Two players buy and build various features on locations

    -> Phase Phase Sim 02
        -> Two players spawn units and use features while attempting to kill other player off
*/