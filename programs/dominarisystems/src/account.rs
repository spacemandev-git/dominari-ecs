use anchor_lang::prelude::*;
use ecs::state::SerializedComponent;

use crate::state::*;


#[account]
pub struct SystemConfig {
    pub authority: Pubkey,
    pub components: RelevantComponentKeys
}

/**
 * Blueprints are preloaded set of components to initalize an Entity
 */
#[account]
pub struct Blueprint {
    pub entity_name: String, // Max Size 128 Bytes
    pub components: Vec<SerializedComponent>
}