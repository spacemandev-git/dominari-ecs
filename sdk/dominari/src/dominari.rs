use anchor_lang::{prelude::*, InstructionData};
use anchor_lang::system_program::ID as system_program;
use dominarisystems::state::RelevantComponentKeys;
use ecs::account::Entity;
use solana_client_wasm::WasmClient;
use solana_sdk::instruction::Instruction;
use rand::Rng;
use crate::util::fetch_account;
use crate::world::World;

pub struct Dominari {
    pub client: WasmClient,
    pub world: Pubkey,
}

impl Dominari {
    pub fn new(rpc: &str, world: Pubkey) -> Self {
        return Dominari {
            client: WasmClient::new(rpc),
            world,
        }
    }

    pub fn get_system_signer(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"System_Signer"], &dominarisystems::id()).0
    }

    pub async fn init_map(&self, payer:Pubkey, instance:u64, max_x:u8, max_y:u8) -> Vec<Instruction> {
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
            dominarisystems::id().to_bytes().as_ref()
        ], &world_program).0;

        let mut rng = rand::thread_rng();
        let entity_id:u64 = rng.gen();
        println!("Map Entity ID: {:#}", entity_id);

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

    pub async fn get_map_by_instance(&self, instance:u64) -> Entity {

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            self.world.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;

        let map_entity = Pubkey::find_program_address(&[
            b"Entity".as_ref(),
            instance.to_be_bytes().as_ref(),
            world_instance.as_ref()
        ], &ecs::id()).0;

        fetch_account(&self.client, &map_entity).await.unwrap()
    } 
}

pub struct ComponentSchema {
    pub schemas: bimap::BiMap<String, Pubkey>
}

impl ComponentSchema {
    pub fn new(world:&World) -> Self {
        let mut schemas = bimap::BiMap::<String, Pubkey>::new();
        let urls =  ComponentSchema::get_all_schema_urls();

        for url in urls.iter() {
            schemas.insert(url.clone(), ComponentSchema::get_world_component(world, url));
        }

        return ComponentSchema { schemas, }
    }

    pub fn get_world_component(world:&World, schema: &String) -> Pubkey {
        Pubkey::find_program_address(&[schema.as_bytes().as_ref()], &world.pubkey).0
    }

    pub fn get_component_pubkey(&self, schema: &String) -> &Pubkey {
        self.schemas.get_by_left(schema).unwrap()
    }

    pub fn get_component_url(&self, pubkey: &Pubkey) -> &String {
        self.schemas.get_by_right(pubkey).unwrap()
    }

    pub fn get_all_schema_urls() -> Vec<String> {
        vec![
            "metadata.json".to_string(),
            "mapmeta.json".to_string(),
            "location.json".to_string(),
            "feature.json".to_string(),
            "owner.json".to_string(),
            "value.json".to_string(),
            "occupant.json".to_string(),
            "player_stats.json".to_string(),
            "last_used_slot.json".to_string(),
            "rank.json".to_string(),
            "range.json".to_string(),
            "drop_table.json".to_string(),
            "uses.json".to_string(),
            "healing_power.json".to_string(),
            "health.json".to_string(),
            "damage.json".to_string(),
            "troop_class.json".to_string(),
            "active.json".to_string(),
        ]
    }

    pub fn get_all_component_keys(&self) -> Vec<Pubkey> {
        let values:Vec<Pubkey> = self.schemas.right_values().cloned().collect();
        return values;
    }

    pub fn get_relevant_component_keys(&self) -> dominarisystems::state::RelevantComponentKeys {
        RelevantComponentKeys {
            metadata: *self.get_component_pubkey(&"metadata.json".to_string()),
            mapmeta: *self.get_component_pubkey(&"mapmeta.json".to_string()),
            location: *self.get_component_pubkey(&"location.json".to_string()),
            feature: *self.get_component_pubkey(&"feature.json".to_string()),
            owner: *self.get_component_pubkey(&"owner.json".to_string()),
            value: *self.get_component_pubkey(&"value.json".to_string()),
            occupant: *self.get_component_pubkey(&"occupant.json".to_string()),
            player_stats: *self.get_component_pubkey(&"player_stats.json".to_string()),
            last_used: *self.get_component_pubkey(&"last_used.json".to_string()),
            rank: *self.get_component_pubkey(&"rank.json".to_string()),
            range: *self.get_component_pubkey(&"range.json".to_string()),
            drop_table: *self.get_component_pubkey(&"drop_table.json".to_string()),
            uses: *self.get_component_pubkey(&"uses.json".to_string()),
            healing_power: *self.get_component_pubkey(&"healing_power.json".to_string()),
            health: *self.get_component_pubkey(&"health.json".to_string()),
            damage: *self.get_component_pubkey(&"damage.json".to_string()),
            troop_class: *self.get_component_pubkey(&"troop_class.json".to_string()),
            active: *self.get_component_pubkey(&"active.json".to_string())
        }
    }
    
}