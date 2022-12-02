use std::rc::Rc;
use std::sync::mpsc::channel;
use anchor_client::{solana_sdk::commitment_config::CommitmentConfig, Cluster, EventContext};
use dominari::gamestate::GameState;
use prettytable::{Table, Cell};
use std::sync::{Arc};
use futures::lock::Mutex;

use crate::*;

pub async fn game_repl(client: &mut Client, instance: u64) {
    client.dominari.build_gamestate(instance).await;
    //println!("Game Index: {:?}", client.dominari.get_gamestate(instance).index.as_ref().unwrap());

    // Start Event Listeners
    println!("Starting event listners...");
    let cluster = Cluster::Custom(
        RPC_URL.to_string(),
        RPC_URL.replace("http", "ws").to_string().replace("8899", "8900").to_string(),
    );
    let payer = Rc::new(read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap());
    let anchor = anchor_client::Client::new_with_options(
        cluster,
        payer,
        CommitmentConfig::processed()
    );
    let program = anchor.program(Dominari::id());
    let dominari = Arc::new(Mutex::new(client.dominari.clone()));
    let nus_client = dominari.clone();
    let (send_nus, recv_nus) = channel();

    let handle = program.on(move |_ctx:&EventContext, event: NewUnitSpawned| {
        if event.instance == instance {
            send_nus.send(event).unwrap();
        }
    }).unwrap();
    std::thread::spawn(move || {
        drop(handle);
    });
    tokio::spawn(async move {
        for x in recv_nus {
            if x.instance == instance {
                let mut dom = nus_client.lock().await;
                dom.get_mut_gamestate(instance).update_entity(x.tile).await;
                dom.get_mut_gamestate(instance).update_entity(x.unit).await;
                dom.get_mut_gamestate(instance).update_instance_index().await;
            }
        }
    });

    let tmv_client = dominari.clone();
    let (send_tmv, recv_tmv) = channel();
    let handle = program.on(move |_ctx:&EventContext, event: TroopMovement| {
        if event.instance == instance {
            send_tmv.send(event).unwrap();
        }
    }).unwrap();
    std::thread::spawn(move || {
        drop(handle);
    });
    tokio::spawn(async move {
        for x in recv_tmv {
            if x.instance == instance {
                let mut dom = tmv_client.lock().await;
                dom.get_mut_gamestate(instance).update_entity(x.from).await;
                dom.get_mut_gamestate(instance).update_entity(x.to).await;
                dom.get_mut_gamestate(instance).update_entity(x.unit).await;
                dom.get_mut_gamestate(instance).update_instance_index().await;
            }
        }
    });

    let atk_client = dominari.clone();
    let (send_atk, recv_atk) = channel();
    let handle = program.on(move |_ctx:&EventContext, event: TileAttacked| {
        if event.instance == instance {
            send_atk.send(event).unwrap();
        }
    }).unwrap();
    std::thread::spawn(move || {
        drop(handle);
    });
    tokio::spawn(async move {
        for x in recv_atk {
            if x.instance == instance {
                let mut dom = atk_client.lock().await;
                dom.get_mut_gamestate(instance).update_entity(x.attacker).await;
                dom.get_mut_gamestate(instance).update_entity(x.defender).await;
                dom.get_mut_gamestate(instance).update_instance_index().await;
            }
        }
    });
    println!("Listeners started, awaiting input...");
    // Play Game
    loop {
        let mut input = String::new();
        println!("Input: ");
        std::io::stdin().read_line(&mut input).unwrap();
        let args:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        match args.get(0).unwrap().as_str() {
            // Spawn Unit, Move Unit, Attack Unit
            // Use Features
            // Print (Players, Tile, Feature, Unit)

            // Print Players
            "players" => {
                players_table(dominari.lock().await.state.get(&instance).unwrap()).printstd();
            }
            // Print Map
            "map" => {
                print_map(dominari.lock().await.state.get(&instance).unwrap()).printstd();
            }
            "exit" => {break;},
            &_ => {}
        }
    }
}

pub fn print_map(state: &GameState) -> Table {
    let index = state.index.as_ref().unwrap();
    let mapmeta = state.get_entity_mapmeta(&index.map).unwrap();

    let mut table = Table::new();
    
    let mut first_row = row!["X/Y"];
    for x in 0..mapmeta.max_x {
        first_row.add_cell(Cell::new(format!("{x}").as_str()));
    }
    table.add_row(first_row);

    for y in 0..mapmeta.max_y {
        let mut row = row![format!("{y}").as_str()];
        for x in 0..mapmeta.max_x {
            let mut tile_info:String = String::from("");
            
            let tile = state.get_tile(x, y).unwrap();
            let feature = state.get_feature_on_tile(tile.0);
            let occupant  = state.get_unit_on_tile(tile.0);
            // Show feature name
            if feature.0.is_some() {
                let metadata = state.get_entity_metadata(&feature.0.unwrap()).unwrap();
                tile_info += format!("\nFeat: {}", metadata.name).as_str();
            }
            // Show unit name
            if occupant.0.is_some() {
                let metadata = state.get_entity_metadata(&occupant.0.unwrap()).unwrap();
                tile_info += format!("\nUnit: {}", metadata.name).as_str();
            }
            row.add_cell(Cell::new(tile_info.as_str()));
        }
        table.add_row(row);
    }
    table
}

pub fn players_table(state: &GameState) -> Table {
    let index = state.index.as_ref().unwrap();
    let mut table = Table::new();
    table.add_row(row!["ID", "NAME", "SCORE", "KILLS", "CARDS"]);

    for player_id in index.players.iter() {
        let player_stats = state.get_entity_player_stats(&player_id).unwrap();
        table.add_row(row![
            player_id.to_string(),
            player_stats.name,
            player_stats.score.to_string(),
            player_stats.kills.to_string(),
            format!("{:?}", player_stats.cards)
        ]);
    }
    table
}