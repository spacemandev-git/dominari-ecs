use anchor_lang::prelude::*;
use ecs::state::SerializedComponent;

use crate::state::*;


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
#[account]
pub struct InstanceIndex {
    pub map: Pubkey,
    pub tiles: Vec<Pubkey>,
    pub features: Vec<Pubkey>,
    pub unit: Vec<Pubkey>,
    pub player: Vec<Pubkey>,
}