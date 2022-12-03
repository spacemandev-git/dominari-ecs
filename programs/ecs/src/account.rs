use anchor_lang::prelude::*;
use std::collections::BTreeMap;
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
    pub instance: u64,
    pub world: Pubkey,
    pub world_signer: Pubkey,
    pub components: BTreeMap<Pubkey, SerializedComponent>,
}

#[account] 
pub struct EntityNFT {
    pub entity: Pubkey,
    pub mint: Pubkey,
}