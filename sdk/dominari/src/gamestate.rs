
use std::collections::HashMap;
use anchor_lang::{prelude::Pubkey, Key, AnchorDeserialize};
use dominarisystems::{account::InstanceIndex, component::*};
use ecs::account::Entity;
use solana_client_wasm::WasmClient;
use crate::{ util::*, dominari::{ComponentSchema, Blueprint}, universe::Universe};

#[derive(Clone)]
pub struct GameState {
    pub client: WasmClient,
    pub world: Pubkey,
    pub instance: u64,
    pub index: Option<InstanceIndex>,
    pub entities: Option<HashMap<u64, Entity>>,
    pub schemas: ComponentSchema,
    pub blueprints: Blueprint,
}

impl GameState {
    pub fn new(client: WasmClient, world: Pubkey, instance:u64) -> Self {
        let blueprints = Blueprint::new();
        GameState { 
            client,
            world,
            instance,
            index: None,
            entities: None,
            schemas: ComponentSchema::new(&world),
            blueprints,
        }
    }

    pub async fn update_instance_index(&mut self) -> dominarisystems::account::InstanceIndex {
        let world_instance = Universe::get_world_instance(self.world, self.instance);
        
        Pubkey::find_program_address(&[
            b"World".as_ref(),
            self.world.as_ref(),
            self.instance.to_be_bytes().as_ref()
        ], &ecs::id()).0;
        
        let pubkey = Pubkey::find_program_address(&[
            b"Instance_Index".as_ref(),
            world_instance.as_ref()
        ], &dominarisystems::id()).0;

        let index:InstanceIndex = fetch_account(&self.client, &pubkey).await.unwrap();
        self.index = Some(index.clone());
        index
    }

    pub async fn update_entity(&mut self, id: u64) {
        let world_instance = Universe::get_world_instance(self.world, self.instance);

        let pubkey = Pubkey::find_program_address(
            &[
                b"Entity",
                id.to_be_bytes().as_ref(),
                world_instance.to_bytes().as_ref()
            ],
            &ecs::id()
        ).0;

        let entity:Entity = fetch_account(&self.client, &pubkey).await.unwrap();
        self.entities.as_mut().unwrap().insert(id, entity);
    }

    pub async fn load_state(&mut self) {
        let world_instance = Universe::get_world_instance(self.world, self.instance);
        self.update_instance_index().await;
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
  
    pub fn get_tile(&self, x:u8, y:u8) -> Result<(u64, Entity), &'static str> {
        if self.index.is_none() {
            return Err("Game state must be loaded first!");
        }

        for id in &self.index.as_ref().unwrap().tiles {
            let tile = self.entities.as_ref().unwrap().get(&id).unwrap();
            let location = self.get_entity_location(id).unwrap();
            if location.x == x && location.y == y {
                return Ok((id.clone(), tile.clone()))
            }
        }
        Err("Tile Not Found!") 
    }

    pub fn get_unit_on_tile(&self, tile_id:u64) -> (Option<u64>, Option<Entity>) {
        let occupant = self.get_entity_occupant(&tile_id).unwrap();
        if occupant.occupant_id.is_none() {
            return (None, None)
        } else {
            let entity = (self.entities.as_ref().unwrap().get(&occupant.occupant_id.unwrap())).unwrap().clone();
            return (occupant.occupant_id, Some(entity))
        }
    }

    pub fn get_feature_on_tile(&self, tile_id:u64) -> (Option<u64>, Option<Entity>) {
        let feature = self.get_entity_feature(&tile_id).unwrap();
        if feature.feature_id.is_none() {
            return (None, None)
        } else {
            let entity = (self.entities.as_ref().unwrap().get(&feature.feature_id.unwrap())).unwrap().clone();
            return (feature.feature_id, Some(entity))
        }
    }

    /** COMPONENT GETTERS */
    pub fn get_entity_metadata(&self, entity_id: &u64) -> Option<ComponentMetadata> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().metadata.key());
        if sc.is_none() { return None };
        Some(ComponentMetadata::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_mapmeta(&self, entity_id: &u64) -> Option<ComponentMapMeta> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().mapmeta.key());
        if sc.is_none() { return None };
        Some(ComponentMapMeta::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_location(&self, entity_id: &u64) -> Option<ComponentLocation> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().location.key());
        if sc.is_none() { return None };
        Some(ComponentLocation::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_feature(&self, entity_id: &u64) -> Option<ComponentFeature> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().feature.key());
        if sc.is_none() { return None };
        Some(ComponentFeature::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_owner(&self, entity_id: &u64) -> Option<ComponentOwner> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().owner.key());
        if sc.is_none() { return None };
        Some(ComponentOwner::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_value(&self, entity_id: &u64) -> Option<ComponentValue> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().value.key());
        if sc.is_none() { return None };
        Some(ComponentValue::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_occupant(&self, entity_id: &u64) -> Option<ComponentOccupant> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().occupant.key());
        if sc.is_none() { return None };
        Some(ComponentOccupant::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_player_stats(&self, entity_id: &u64) -> Option<ComponentPlayerStats> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().player_stats.key());
        if sc.is_none() { return None };
        Some(ComponentPlayerStats::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_last_used(&self, entity_id: &u64) -> Option<ComponentLastUsed> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().last_used.key());
        if sc.is_none() { return None };
        Some(ComponentLastUsed::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_feature_rank(&self, entity_id: &u64) -> Option<ComponentFeatureRank> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().feature_rank.key());
        if sc.is_none() { return None };
        Some(ComponentFeatureRank::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_range(&self, entity_id: &u64) -> Option<ComponentRange> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().range.key());
        if sc.is_none() { return None };
        Some(ComponentRange::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_drop_table(&self, entity_id: &u64) -> Option<ComponentDropTable> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().drop_table.key());
        if sc.is_none() { return None };
        Some(ComponentDropTable::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_uses(&self, entity_id: &u64) -> Option<ComponentUses> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().uses.key());
        if sc.is_none() { return None };
        Some(ComponentUses::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_healing_power(&self, entity_id: &u64) -> Option<ComponentHealingPower> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().healing_power.key());
        if sc.is_none() { return None };
        Some(ComponentHealingPower::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_health(&self, entity_id: &u64) -> Option<ComponentHealth> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().health.key());
        if sc.is_none() { return None };
        Some(ComponentHealth::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_damage(&self, entity_id: &u64) -> Option<ComponentDamage> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().damage.key());
        if sc.is_none() { return None };
        Some(ComponentDamage::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_troop_class(&self, entity_id: &u64) -> Option<ComponentTroopClass> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().troop_class.key());
        if sc.is_none() { return None };
        Some(ComponentTroopClass::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_active(&self, entity_id: &u64) -> Option<ComponentActive> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().active.key());
        if sc.is_none() { return None };
        Some(ComponentActive::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_cost(&self, entity_id: &u64) -> Option<ComponentCost> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().cost.key());
        if sc.is_none() { return None };
        Some(ComponentCost::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_offchain_metadata(&self, entity_id: &u64) -> Option<ComponentOffchainMetadata> {
        let serialized_components = &self.entities.as_ref().unwrap().get(&entity_id).unwrap().components;
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().offchain_metadata.key());
        if sc.is_none() { return None };
        Some(ComponentOffchainMetadata::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
}