use anchor_lang::prelude::*;

use crate::state::*;


#[account]
pub struct WorldInstance {
    pub world: Pubkey, 
    pub instance: u64,
}


#[account]
pub struct Entity {
    pub world: Pubkey, // Acts as the Update Authority
    pub instance: u64,
    pub mint: Pubkey, //Ties the Entity to a Token Mint
    pub components: Vec<SerializedComponent>
}