use anchor_lang::prelude::*;

use crate::state::*;


#[account]
pub struct WorldInstance {
    pub world: Pubkey, 
    pub instance: u64,
    pub entities: u64,
}


#[account]
pub struct Entity {
    pub entity_id: u64,
    pub world: Pubkey, // Acts as the Update Authority
    pub benefactor: Pubkey, // Key that gets the $ when the PDA is closed
    pub instance: u64,
    pub components: Vec<SerializedComponent>,
}