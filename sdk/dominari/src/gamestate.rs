
use std::collections::HashMap;
use anchor_lang::{prelude::Pubkey, Key, AnchorDeserialize};
use dominarisystems::{account::InstanceIndex, component::ComponentLocation};
use ecs::account::Entity;
use solana_client_wasm::WasmClient;
use crate::{ util::*, dominari::ComponentSchema, universe::Universe};

#[derive(Clone)]
pub struct GameState {
    pub client: WasmClient,
    pub world: Pubkey,
    pub instance: u64,
    pub index: Option<InstanceIndex>,
    pub entities: Option<HashMap<u64, Entity>>,
    pub schemas: ComponentSchema,
}

impl GameState {
    pub fn new(client: WasmClient, world: Pubkey, instance:u64) -> Self {
        GameState { 
            client,
            world,
            instance,
            index: None,
            entities: None,
            schemas: ComponentSchema::new(&world)
        }
    }

    pub async fn get_instance_index(&self, instance:u64) -> dominarisystems::account::InstanceIndex {
        let world_instance = Universe::get_world_instance(self.world, self.instance);
        
        Pubkey::find_program_address(&[
            b"World".as_ref(),
            self.world.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;
        
        let pubkey = Pubkey::find_program_address(&[
            b"Instance_Index".as_ref(),
            world_instance.as_ref()
        ], &dominarisystems::id()).0;

        fetch_account(&self.client, &pubkey).await.unwrap()
    }


    pub async fn load_state(&mut self) {
        let world_instance = Universe::get_world_instance(self.world, self.instance);
        let index = self.get_instance_index(self.instance).await;
        self.index = Some(index);
        let mut entities: HashMap<u64, Entity> = HashMap::new();


        entities.insert(
            self.index.as_ref().unwrap().map,
            fetch_accounts::<Entity>(
                &self.client,
                &Universe::get_keys_from_id(
                    world_instance,
                    vec![self.index.as_ref().unwrap().map]
                )
            ).await.get(0).unwrap().1.to_owned()
        );        
        
        let tile_entities:Vec<(Pubkey, Entity)> = fetch_accounts::<Entity>(&self.client, &Universe::get_keys_from_id(world_instance, self.index.as_ref().unwrap().tiles.clone())).await;
        for (i, e) in tile_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().tiles.get(i).unwrap(), e.1.to_owned());
        }

        let feature_entities:Vec<(Pubkey, Entity)> = fetch_accounts(&self.client, &Universe::get_keys_from_id(world_instance, self.index.as_ref().unwrap().features.clone())).await;
        for (i, e) in feature_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().features.get(i).unwrap(), e.1.to_owned());
        }

        let unit_entities:Vec<(Pubkey, Entity)> = fetch_accounts(&self.client, &Universe::get_keys_from_id(world_instance, self.index.as_ref().unwrap().units.clone())).await;
        for (i, e) in unit_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().units.get(i).unwrap(), e.1.to_owned());
        }

        let player_entities:Vec<(Pubkey, Entity)> = fetch_accounts(&self.client, &Universe::get_keys_from_id(world_instance, self.index.as_ref().unwrap().players.clone())).await;
        for (i, e) in player_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().players.get(i).unwrap(), e.1.to_owned());
        }

        self.entities = Some(entities);
    }
  
    pub fn get_tile(&self, entities: &Vec<u64>, x:u8, y:u8) -> Result<(u64, Entity), &'static str> {
        if self.index.is_none() {
            return Err("Game state must be loaded first!");
        }

        for id in entities {
            let tile = self.entities.as_ref().unwrap().get(id).unwrap();
            let location_component = tile.components.iter().find(|component| {
                component.component_key.key() == self.schemas.get_component_pubkey(&"location".to_string()).key()    
            }).unwrap(); 
            let location:ComponentLocation = ComponentLocation::try_from_slice(location_component.data.as_slice()).unwrap();
            if location.x == x && location.y == y {
                return Ok((id.clone(), tile.clone()))
            }
        }
        Err("Tile Not Found!") 
    }

}