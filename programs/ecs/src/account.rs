use anchor_lang::prelude::*;

use crate::state::*;


#[account]
pub struct WorldInstance {
    pub world: Pubkey, 
    pub instance: u64,
}


#[account]
pub struct Entity {
    pub authority: Pubkey,
    pub world: Pubkey, // Acts as the Update Authority
    pub mint: Pubkey, //Ties the Entity to a Token Mint
    pub instance: u64,
    pub components: Vec<SerializedComponent>
}