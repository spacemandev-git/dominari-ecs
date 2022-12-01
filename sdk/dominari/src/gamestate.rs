
use std::collections::HashMap;
use anchor_lang::{prelude::Pubkey, Key, AnchorDeserialize};
use dominarisystems::{account::InstanceIndex, component::*};
use ecs::{account::Entity, state::SerializedComponent};
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

    pub fn get_unit_on_tile(&self, tile_id:u64) -> (Option<u64>, Option<Entity>) {
        let tile = self.entities.as_ref().unwrap().get(&tile_id).unwrap();
        let occupant_c = tile.components.iter().find(|&comp| comp.component_key == self.schemas.key_index.as_ref().unwrap().occupant.key()).unwrap();
        let occupant = ComponentOccupant::try_from_slice(&occupant_c.data.as_slice()).unwrap();
        if occupant.occupant_id.is_none() {
            return (None, None)
        } else {
            let entity = (self.entities.as_ref().unwrap().get(&occupant.occupant_id.unwrap())).unwrap().clone();
            return (occupant.occupant_id, Some(entity))
        }
    }


    /** COMPONENT GETTERS */
    pub fn get_entity_metadata(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentMetadata> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().metadata.key());
        if sc.is_none() { return None };
        Some(ComponentMetadata::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_mapmeta(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentMapMeta> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().mapmeta.key());
        if sc.is_none() { return None };
        Some(ComponentMapMeta::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_location(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentLocation> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().location.key());
        if sc.is_none() { return None };
        Some(ComponentLocation::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_feature(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentFeature> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().feature.key());
        if sc.is_none() { return None };
        Some(ComponentFeature::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_owner(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentOwner> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().owner.key());
        if sc.is_none() { return None };
        Some(ComponentOwner::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_value(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentValue> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().value.key());
        if sc.is_none() { return None };
        Some(ComponentValue::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_occupant(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentOccupant> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().occupant.key());
        if sc.is_none() { return None };
        Some(ComponentOccupant::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_player_stats(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentPlayerStats> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().player_stats.key());
        if sc.is_none() { return None };
        Some(ComponentPlayerStats::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_last_used(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentLastUsed> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().last_used.key());
        if sc.is_none() { return None };
        Some(ComponentLastUsed::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_feature_rank(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentFeatureRank> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().feature_rank.key());
        if sc.is_none() { return None };
        Some(ComponentFeatureRank::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_range(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentRange> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().range.key());
        if sc.is_none() { return None };
        Some(ComponentRange::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_drop_table(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentDropTable> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().drop_table.key());
        if sc.is_none() { return None };
        Some(ComponentDropTable::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_uses(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentUses> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().uses.key());
        if sc.is_none() { return None };
        Some(ComponentUses::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_healing_power(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentHealingPower> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().healing_power.key());
        if sc.is_none() { return None };
        Some(ComponentHealingPower::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_health(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentHealth> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().health.key());
        if sc.is_none() { return None };
        Some(ComponentHealth::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_damage(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentDamage> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().damage.key());
        if sc.is_none() { return None };
        Some(ComponentDamage::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_troop_class(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentTroopClass> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().troop_class.key());
        if sc.is_none() { return None };
        Some(ComponentTroopClass::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_active(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentActive> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().active.key());
        if sc.is_none() { return None };
        Some(ComponentActive::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_cost(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentCost> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().cost.key());
        if sc.is_none() { return None };
        Some(ComponentCost::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_offchain_metadata(&self, serialized_components: &Vec<SerializedComponent>) -> Option<ComponentOffchainMetadata> {
        let sc = serialized_components.iter().find(|&c| c.component_key == self.schemas.key_index.as_ref().unwrap().offchain_metadata.key());
        if sc.is_none() { return None };
        Some(ComponentOffchainMetadata::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
}