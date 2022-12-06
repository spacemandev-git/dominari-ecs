use std::rc::Rc;
use std::sync::mpsc::channel;
use anchor_client::{solana_sdk::{commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction}, Cluster, EventContext};
use dominari::gamestate::GameState;
use prettytable::{Table, Cell};
use std::sync::Arc;
use futures::lock::Mutex;

use crate::*;

pub async fn game_repl(client: &mut Client, instance: u64) {
    // Build Gamestate
    client.dominari.build_gamestate(&client.rpc, instance).await;
    // Grab all blueprints from various folders
    let mut blueprint_names: Vec<String> = vec![];
    let blueprint_paths = vec!["blueprints/features", "blueprints/mods", "blueprints/units"];
    for bpath in blueprint_paths {
        for path in fs::read_dir(bpath).unwrap().into_iter() {
            let pathspec = path.as_ref().unwrap().path().display().to_string().replace(".toml", "").to_string();
            let name = pathspec.split("/").collect::<Vec<&str>>().pop().unwrap();
            blueprint_names.push(name.to_string());
        }
    }
    client.dominari.get_mut_gamestate(instance).blueprints.insert_blueprint_strings(&blueprint_names);
    
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
                let player = dom.get_mut_gamestate(instance).get_entity_owner(&x.unit).unwrap().player.unwrap();
                dom.get_mut_gamestate(instance).update_entity(player).await;
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
                dom.get_mut_gamestate(instance).update_entity(x.defending_tile).await;
                dom.get_mut_gamestate(instance).update_instance_index().await;
                println!("Damage dealt: {}", x.damage);
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
            // Use Features
            "time" => {
                println!("Slot: {}", client.rpc.get_slot().await.unwrap());
            }
            "refresh" => {
                dominari.lock().await.get_mut_gamestate(instance).load_state().await;
            }
            "tile" => {
                // tile <x> <y>
                let x:u8 = args.get(1).unwrap().parse().unwrap();
                let y:u8 = args.get(2).unwrap().parse().unwrap();
                tile_info(
                    dominari.lock().await.state.get(&instance).unwrap(),
                    x,
                    y
                );
            }
            "attack" => {
                // attack <from x> <from y> <to x> <to y>
                let from_x:u8 = args.get(1).unwrap().parse().unwrap();
                let from_y:u8 = args.get(2).unwrap().parse().unwrap();
                let to_x:u8 = args.get(3).unwrap().parse().unwrap();
                let to_y:u8 = args.get(4).unwrap().parse().unwrap();
                attack_tile(
                    client,
                    dominari.lock().await.state.get(&instance).unwrap(),
                    from_x,
                    from_y,
                    to_x,
                    to_y
                ).await;
            }
            "move" => {
                // move <from x> <from y> <to x> <to y>
                let from_x:u8 = args.get(1).unwrap().parse().unwrap();
                let from_y:u8 = args.get(2).unwrap().parse().unwrap();
                let to_x:u8 = args.get(3).unwrap().parse().unwrap();
                let to_y:u8 = args.get(4).unwrap().parse().unwrap();
                move_unit(
                    client,
                    dominari.lock().await.state.get(&instance).unwrap(),
                    from_x,
                    from_y,
                    to_x,
                    to_y
                ).await;
            }
            "spawn" => {
                // spawn <player_id> <x> <y> <troop/mod name>
                let player_id:u64 = args.get(1).unwrap().parse().unwrap();
                let x:u8 = args.get(2).unwrap().parse().unwrap();
                let y:u8 = args.get(3).unwrap().parse().unwrap();
                let card = args.get(4).unwrap();
                spawn(
                    client,
                    dominari.lock().await.state.get(&instance).unwrap(),
                    player_id,
                    x,
                    y,
                    card
                ).await;
            }   
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
                tile_info += format!("\n{}", metadata.name).as_str();
            }
            // Show unit name
            if occupant.0.is_some() {
                let metadata = state.get_entity_metadata(&occupant.0.unwrap()).unwrap();
                tile_info += format!("\n{}", metadata.name).as_str();

                // Print Unit Owner underneath
                let owner = state.get_entity_owner(&occupant.0.unwrap()).unwrap();
                let player = state.get_entity_player_stats(&owner.player.unwrap()).unwrap();
                tile_info += format!("\n{}", player.name).as_str();
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
            format!("{:?}",
                player_stats.cards
                    .iter()
                    .map(|key| {state.blueprints.get_blueprint_by_key(key).unwrap()})
                    .collect::<Vec<String>>()
            )
        ]);
    }
    table
}

pub fn tile_info(state: &GameState, x:u8, y:u8) {
    let tile = state.get_tile(x,y).unwrap();
    let feature = state.get_feature_on_tile(tile.0);
    let occupant  = state.get_unit_on_tile(tile.0);

    println!("\n Tile: ({x},{y}) ID {}", tile.0);
    if feature.0.is_some() {
        println!("\tFeature:");
        // Feature should print various components based on type of Feature TODO
        let metadata = state.get_entity_metadata(&feature.0.unwrap()).unwrap();
        println!("\t{}", metadata.name);
    }
    // Show unit name
    if occupant.0.is_some() {
        println!("\tUnit:");
        let metadata = state.get_entity_metadata(&occupant.0.unwrap()).unwrap();
        println!("\t{}", metadata.name);

        // Print Unit Owner underneath
        let owner = state.get_entity_owner(&occupant.0.unwrap()).unwrap();
        let player = state.get_entity_player_stats(&owner.player.unwrap()).unwrap();
        println!("\t{}", player.name);

        // Health
        println!("\t{:?}", state.get_entity_health(&occupant.0.unwrap()).unwrap());
        // Damage
        println!("\t{:?}", state.get_entity_damage(&occupant.0.unwrap()).unwrap());
        // Class
        println!("\t{:?}", state.get_entity_troop_class(&occupant.0.unwrap()).unwrap());
        // Range
        println!("\t{:?}", state.get_entity_range(&occupant.0.unwrap()).unwrap());
        // Value
        println!("\t{:?}", state.get_entity_value(&occupant.0.unwrap()).unwrap());
        // Last Used
        println!("\t{:?}", state.get_entity_last_used(&occupant.0.unwrap()).unwrap());
        // Active
        println!("\t{:?}", state.get_entity_active(&occupant.0.unwrap()).unwrap());


    }
}

pub async fn move_unit(client: &Client, state: &GameState, from_x: u8, from_y: u8, to_x: u8, to_y: u8) {
    let from_tile = state.get_tile(from_x, from_y).unwrap();
    let to_tile = state.get_tile(to_x, to_y).unwrap();
    let mut move_unit_tx = Transaction::new_with_payer(
        client.dominari.move_unit(
            client.id01.pubkey(),
            state.instance,
            from_tile.0,
            to_tile.0
        ).as_slice(),
        Some(&client.id01.pubkey())
    );
    move_unit_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    //client.rpc.send_and_confirm_transaction(&move_unit_tx).await.unwrap();
    send_tx_skip_preflight(move_unit_tx);
}

// Doesn't support attacking Features yet.
pub async fn attack_tile(client: &Client, state: &GameState, from_x: u8, from_y: u8, to_x: u8, to_y: u8) {
    let from_tile = state.get_tile(from_x, from_y).unwrap();
    let to_tile = state.get_tile(to_x, to_y).unwrap();
    
    let attacker = state.get_entity_occupant(&from_tile.0).unwrap().occupant_id.unwrap();
    let defender = state.get_entity_occupant(&to_tile.0).unwrap().occupant_id.unwrap();

    let mut atk_ix = client.dominari.attack_tile(
        client.id01.pubkey(),
        state.instance,
        attacker,
        defender,
        to_tile.0
    );
    atk_ix.insert(0, ComputeBudgetInstruction::set_compute_unit_limit(400_000));

    let mut atk_tile_tx = Transaction::new_with_payer(
        atk_ix.as_slice(),
        Some(&client.id01.pubkey())
    );
    atk_tile_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
    client.rpc.send_and_confirm_transaction(&atk_tile_tx).await.unwrap();
    //send_tx_skip_preflight(atk_tile_tx);
}

pub async fn spawn(client: &Client, state: &GameState, player_id:u64, x:u8, y:u8, card: &String) {
    let tile = state.get_tile(x, y).unwrap();
    let occupant = state.get_entity_occupant(&tile.0).unwrap().occupant_id;
    // Check if Tile is EMPTY == Use SPAWN UNIT
    // If tile is OCCUPIED == USE MODIFY UNIT

    if occupant.is_none() {
        // SPAWN UNIT
        let compute_buget_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);
        let mut spawn_ix = client.dominari.spawn_unit(
            client.id01.pubkey(),
            state.instance,
            player_id,
            tile.0,
            state.blueprints.get_blueprint_by_name(card).unwrap()
        );

        spawn_ix.insert(0, compute_buget_ix); 

        let mut spawn_unit_tx = Transaction::new_with_payer(
            &spawn_ix.as_slice(),
            Some(&client.id01.pubkey())
        );
        spawn_unit_tx.sign(&[&client.id01], client.rpc.get_latest_blockhash().await.unwrap());
        client.rpc.send_and_confirm_transaction(&spawn_unit_tx).await.unwrap();
        //send_tx_skip_preflight(spawn_unit_tx);
    } else {
        // Check if unit belongs to the player_id
        // If it does, play the modify_unit tx
    }
}