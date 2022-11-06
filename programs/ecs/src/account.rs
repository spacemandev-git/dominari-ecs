use anchor_lang::prelude::*;

use crate::state::*;


#[account]
pub struct WorldInstance {
    pub world: Pubkey, 
    pub instance: u64,
}


#[account]
pub struct Entity {
    pub world: Pubkey,
    pub instance: u64,
    pub components: Vec<SerializedComponent>
}