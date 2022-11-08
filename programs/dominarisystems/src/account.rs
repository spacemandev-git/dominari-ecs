use anchor_lang::prelude::*;
use ecs::state::SerializedComponent;

use crate::state::*;


#[account]
pub struct SystemConfig {}

/**
 * Blueprints are preloaded set of components to initalize an Entity
 */
#[account]
pub struct Blueprint {
    pub components: Vec<SerializedComponent>
}