use anchor_lang::prelude::*;
use ecs::state::SerializedComponent;

use crate::{state::*, component::MaxSize};


#[account]
pub struct SystemConfig {
    pub authority: Pubkey,
    pub components: RelevantComponentKeys,
}

/**
 * Blueprints are preloaded set of components to initalize an Entity
 */
#[account]
pub struct Blueprint {
    pub name: String,
    pub components: Vec<SerializedComponent>
}

/**
 * Always needs a map for an instance
 * Init during Init Map
 * Then realloc+ on entity spawn
 */
#[derive(Debug)]
#[account]
pub struct InstanceIndex {
    pub config: GameConfig,
    pub map: u64,
    pub tiles: Vec<u64>,
    pub features: Vec<u64>,
    pub units: Vec<u64>,
    pub players: Vec<u64>,
}

/**
 * DOES NOT INCLUDE GAME CONFIG SIZE
 * To fetch that, use the get_max_size() function on the config object
 * This is because it's dynamically allocated based on # of starting cards passed in
 */
impl MaxSize for InstanceIndex {
    fn get_max_size() -> u64 {
        return 8+4+4+4+4;
    }
}