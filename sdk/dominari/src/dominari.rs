use std::collections::HashMap;
use std::collections::BTreeMap;
use anchor_lang::{prelude::*, InstructionData};
use anchor_lang::system_program::ID as system_program;
use dominarisystems::state::RelevantComponentKeys;
use ecs::state::SerializedComponent;
use solana_client_wasm::WasmClient;
use solana_sdk::instruction::Instruction;
use rand::Rng;
use crate::gamestate::GameState;
use crate::universe::Universe;
use serde::Deserialize;

#[derive(Clone)]
pub struct Dominari {
    pub world: Pubkey,
    pub state: HashMap<u64, GameState>, //maps instanceID to GameState Object
}

impl Dominari {
    pub fn new(world: Pubkey) -> Self {
        return Dominari {
            world,
            state: HashMap::new(),
        }
    }

    pub fn id() -> Pubkey {
        return dominarisystems::id();
    }

    pub fn get_system_signer(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"System_Signer"], &dominarisystems::id()).0
    }

    pub fn init_action_bundle(&self, payer:Pubkey) -> Vec<Instruction> {
        let component_keys = (ComponentSchema::new(&self.world)).get_relevant_component_keys();

        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::Initialize {
                payer,
                system_program,
                system_signer: self.get_system_signer()
            }.to_account_metas(None),
            data: dominarisystems::instruction::Initialize {
                component_keys,
            }.data()
        }]
    }

    pub fn init_map(&self, payer:Pubkey, instance:u64, max_x:u8, max_y:u8) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0;

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let mut rng = rand::thread_rng();
        let entity_id:u64 = rng.gen();

        let map_entity = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            entity_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        vec![Instruction{
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::SystemInitMap {
                payer,
                system_program,
                system_signer,
                world_config,
                world_program,
                universe,
                system_registration,
                world_instance,
                map_entity,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::SystemInitalizeMap {
                entity_id,
                max_x,
                max_y,
            }.data()
        }]
    }

    pub fn init_game(&self, payer:Pubkey, instance: u64, config: dominarisystems::state::GameConfig) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0;

        let universe = ecs::id();

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        let instance_authority = Pubkey::find_program_address(&[
            b"Instance_Authority",
            world_instance.key().as_ref()
        ], &world_program).0;

        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::CreateGameInstance {
                payer,
                system_program,
                system_signer,
                world_config,
                world_program,
                universe,
                world_instance,
                instance_index,
                instance_authority,
            }.to_account_metas(None),
            data: dominarisystems::instruction::CreateGameInstance {
                instance,
                config
            }.data()
        }]
    }

    pub fn init_tile(&self, payer:Pubkey, instance:u64, x:u8, y:u8, cost:u64) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0;

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let mut rng = rand::thread_rng();
        let entity_id:u64 = rng.gen();

        let tile_entity = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            entity_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        vec![Instruction{
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::SystemInitTile {
                payer,
                system_program,
                system_signer,
                world_config,
                world_program,
                universe,
                system_registration,
                world_instance,
                tile_entity,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::SystemInitTile {
                entity_id,
                x,
                y,
                cost
            }.data()
        }]
    }

    pub fn init_feature(&self, payer:Pubkey, instance:u64, tile_id:u64 ,  blueprint: Pubkey) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0;

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let mut rng = rand::thread_rng();
        let entity_id:u64 = rng.gen();

        let feature_entity = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            entity_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        vec![
        //ComputeBudgetInstruction::set_compute_unit_limit(1_400_000u32),    
        Instruction{
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::SystemInstanceFeature {
                payer,
                system_program,
                system_signer,
                world_config,
                world_program,
                universe,
                system_registration,
                world_instance,
                feature_entity,
                blueprint,
                tile_entity: Universe::get_keys_from_id(world_instance, vec![tile_id]).get(0).unwrap().clone(),
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::SystemInstanceFeature {
                entity_id,
            }.data()
        }]
    }

    pub fn get_blueprint_key(blueprint: &String) -> Pubkey {
        Pubkey::find_program_address(&[
            b"Blueprint",
            blueprint.as_bytes().as_ref()
        ], &dominarisystems::id()).0
    }

    pub async fn register_blueprint(&self,payer:Pubkey, name: String, components: BTreeMap<Pubkey, SerializedComponent>) -> Vec<Instruction> {
        let system_signer = self.get_system_signer();
        
        let blueprint = Dominari::get_blueprint_key(&name);
        println!("Registering Blueprint {} at Key {}", name, blueprint);

        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::RegisterBlueprint {
                payer,
                system_program,
                system_config: system_signer,
                blueprint
            }.to_account_metas(None),
            data: dominarisystems::instruction::RegisterBlueprint {
                name,
                components,
            }.data()
        }]
    }

    pub fn init_player(&self, payer:Pubkey, instance: u64, name: String, image: String) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0; 

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let mut rng = rand::thread_rng();
        let entity_id:u64 = rng.gen();

        let player_entity = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            entity_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;
        
        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::SystemInitPlayer {
                payer,
                system_program,
                system_signer,

                world_config,
                world_program,
                universe,

                system_registration,
                world_instance,

                player_entity,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::SystemInitPlayer {
                entity_id,
                name, 
                image
            }.data()
        }]

    }

    pub fn change_game_state(&self, payer: Pubkey, instance: u64, player_id: u64, game_state: dominarisystems::account::PlayPhase) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0;

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        let map = Universe::get_keys_from_id(world_instance, vec![self.get_gamestate(instance).index.as_ref().unwrap().map]).get(0).unwrap().clone();
        let player = Universe::get_keys_from_id(world_instance, vec![player_id]).get(0).unwrap().clone();


        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::ChangeGameState {
                payer,
                system_signer,
                world_config,
                world_program,
                universe,
                system_registration,
                world_instance,
                player,
                map,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::ChangeGameState {
                game_state
            }.data()
        }]
    }

    // Spawn Unit
    pub fn spawn_unit(&self, payer:Pubkey, instance:u64, player_id: u64, tile_id:u64, unit_blueprint: Pubkey) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0; 

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let mut rng = rand::thread_rng();
        let unit_id:u64 = rng.gen();

        let unit = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            unit_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let player = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            player_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let tile = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            tile_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;


        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::SpawnUnit {
                payer,
                system_program,
                system_signer,

                world_config,
                world_program,
                universe,

                system_registration,
                world_instance,

                unit,
                unit_blueprint,
                player,
                tile,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::SpawnUnit {
                unit_id
            }.data()
        }]
    }

    // Move Unit
    pub fn move_unit(&self, payer: Pubkey, instance: u64, from_tile_id: u64, to_tile_id: u64) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0; 

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let unit_id = &self.get_gamestate(instance).get_unit_on_tile(from_tile_id).0;
        
        let unit = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            unit_id.unwrap().to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let from = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            from_tile_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let to = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            to_tile_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::MoveUnit {
                payer,
                system_program,
                system_signer,

                world_config,
                world_program,
                universe,

                system_registration,
                world_instance,

                from,
                to,
                unit,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::MoveUnit {}.data()
        }]
    }

    // Attack Unit
    pub fn attack_tile(&self, payer: Pubkey, instance: u64, attacker_id: u64, defender_id: u64, defending_tile_id: u64) -> Vec<Instruction> {
        let world_program = self.world;
        let system_signer = self.get_system_signer();
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &world_program).0; 

        let universe = ecs::id();
        
        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            world_program.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            self.get_system_signer().as_ref()
        ], &world_program).0;

        let attacker = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            attacker_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let defender = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            defender_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let defending_tile = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            defending_tile_id.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            b"Instance_Index",
            world_instance.key().as_ref()
        ], &dominarisystems::id()).0;

        vec![Instruction {
            program_id: dominarisystems::id(),
            accounts: dominarisystems::accounts::AttackTile {
                payer,
                system_program,
                system_signer,

                world_config,
                world_program,
                universe,

                system_registration,
                world_instance,

                attacker,
                defender,
                defending_tile,
                instance_index
            }.to_account_metas(Some(true)),
            data: dominarisystems::instruction::AttackTile {}.data()
        }]
    }

    pub async fn build_gamestate(&mut self, client: &WasmClient, instance:u64) -> &GameState {
        self.state.insert(instance, GameState::new(client.clone(), self.world, instance));
        self.get_mut_gamestate(instance).load_state().await;
        self.get_gamestate(instance)
    }

    pub fn get_mut_gamestate(&mut self, instance:u64) -> &mut GameState {
        self.state.get_mut(&instance).unwrap()
    }

    pub fn get_gamestate(&self, instance:u64) -> &GameState {
        self.state.get(&instance).unwrap()
    }
    
}

#[derive(Clone, Debug)]
pub struct ComponentSchema {
    pub schemas: bimap::BiMap<String, Pubkey>,
    pub key_index: Option<RelevantComponentKeys>
}

impl ComponentSchema {
    pub fn new(world:&Pubkey) -> Self {
        let mut schemas = bimap::BiMap::<String, Pubkey>::new();
        let urls =  ComponentSchema::get_all_schema_urls();

        for url in urls.iter() {
            schemas.insert(url.clone(), ComponentSchema::get_world_component(world, url));
        }

        let mut schemas = ComponentSchema { schemas, key_index: None };
        schemas.key_index = Some(schemas.get_relevant_component_keys());
        return schemas;
    }

    pub fn get_world_component(world:&Pubkey, schema: &String) -> Pubkey {
        Pubkey::find_program_address(&[schema.as_bytes().as_ref()], &world).0
    }

    pub fn get_component_pubkey(&self, schema: &String) -> &Pubkey {
        self.schemas.get_by_left(schema).unwrap()
    }

    pub fn get_component_url(&self, pubkey: &Pubkey) -> &String {
        self.schemas.get_by_right(pubkey).unwrap()
    }

    // TODO: Change the names to URLs where each of these components can be found
    pub fn get_all_schema_urls() -> Vec<String> {
        vec![
            "metadata".to_string(),
            "mapmeta".to_string(),
            "location".to_string(),
            "feature".to_string(),
            "owner".to_string(),
            "value".to_string(),
            "occupant".to_string(),
            "player_stats".to_string(),
            "last_used".to_string(),
            "feature_rank".to_string(),
            "range".to_string(),
            "drop_table".to_string(),
            "uses".to_string(),
            "healing_power".to_string(),
            "health".to_string(),
            "damage".to_string(),
            "troop_class".to_string(),
            "active".to_string(),
            "cost".to_string(),
            "offchain_metadata".to_string()
        ]
    }

    pub fn get_all_component_keys(&self) -> Vec<Pubkey> {
        let values:Vec<Pubkey> = self.schemas.right_values().cloned().collect();
        return values;
    }

    pub fn get_relevant_component_keys(&self) -> dominarisystems::state::RelevantComponentKeys {
        RelevantComponentKeys {
            metadata: *self.get_component_pubkey(&"metadata".to_string()),
            mapmeta: *self.get_component_pubkey(&"mapmeta".to_string()),
            location: *self.get_component_pubkey(&"location".to_string()),
            feature: *self.get_component_pubkey(&"feature".to_string()),
            owner: *self.get_component_pubkey(&"owner".to_string()),
            value: *self.get_component_pubkey(&"value".to_string()),
            occupant: *self.get_component_pubkey(&"occupant".to_string()),
            player_stats: *self.get_component_pubkey(&"player_stats".to_string()),
            last_used: *self.get_component_pubkey(&"last_used".to_string()),
            feature_rank: *self.get_component_pubkey(&"feature_rank".to_string()),
            range: *self.get_component_pubkey(&"range".to_string()),
            drop_table: *self.get_component_pubkey(&"drop_table".to_string()),
            uses: *self.get_component_pubkey(&"uses".to_string()),
            healing_power: *self.get_component_pubkey(&"healing_power".to_string()),
            health: *self.get_component_pubkey(&"health".to_string()),
            damage: *self.get_component_pubkey(&"damage".to_string()),
            troop_class: *self.get_component_pubkey(&"troop_class".to_string()),
            active: *self.get_component_pubkey(&"active".to_string()),
            cost: *self.get_component_pubkey(&"cost".to_string()),
            offchain_metadata: *self.get_component_pubkey(&"offchain_metadata".to_string()),
        }
    }
    
}

#[derive(Deserialize, Debug)]
pub struct BlueprintConfig {
    pub metadata: Option<dominarisystems::component::ComponentMetadata>,
    pub mapmeta: Option<dominarisystems::component::ComponentMapMeta>,
    pub location: Option<dominarisystems::component::ComponentLocation>,
    pub feature: Option<dominarisystems::component::ComponentFeature>,
    pub owner: Option<dominarisystems::component::ComponentOwner>,
    pub value: Option<dominarisystems::component::ComponentValue>,
    pub occupant: Option<dominarisystems::component::ComponentOccupant>,
    pub player_stats: Option<dominarisystems::component::ComponentPlayerStats>,
    pub last_used: Option<dominarisystems::component::ComponentLastUsed>,
    pub feature_rank: Option<dominarisystems::component::ComponentFeatureRank>,
    pub range: Option<dominarisystems::component::ComponentRange>,
    pub drop_table: Option<dominarisystems::component::ComponentDropTable>,
    pub uses: Option<dominarisystems::component::ComponentUses>,
    pub healing_power: Option<dominarisystems::component::ComponentHealingPower>,
    pub health: Option<dominarisystems::component::ComponentHealth>,
    pub damage: Option<dominarisystems::component::ComponentDamage>,
    pub troop_class: Option<dominarisystems::component::ComponentTroopClass>,
    pub active: Option<dominarisystems::component::ComponentActive>,
    pub cost: Option<dominarisystems::component::ComponentCost>,
    pub offchain_metadata: Option<dominarisystems::component::ComponentOffchainMetadata>,
}

#[derive(Clone, Debug)]
pub struct Blueprint {
    map: bimap::BiMap<String, Pubkey>
}

impl Blueprint {
    pub fn new() -> Self {
        Blueprint {
           map: bimap::BiMap::<String, Pubkey>::new() 
        }
    }

    pub fn insert_blueprint_strings(&mut self, blueprints: &Vec<String>) {
        for print in blueprints {
            let key = Pubkey::find_program_address(&[
                b"Blueprint",
                print.as_bytes().as_ref()
            ], &dominarisystems::id()).0;
            self.map.insert(print.clone(), key);
        }   
    }

    pub fn get_blueprint_by_key(&self, key: &Pubkey) -> Option<String> {
        self.map.get_by_right(key).cloned()
    }

    pub fn get_blueprint_by_name(&self, name: &String) -> Option<Pubkey> {
        self.map.get_by_left(name).cloned()
    }

}

pub use dominarisystems::account::PlayPhase;
pub use dominarisystems::component::*;
pub use dominarisystems::state::*;
pub use dominarisystems::event::*;