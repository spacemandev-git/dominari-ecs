use anchor_lang::prelude::*;

#[event]
pub struct NewWorldInitalized{
    pub world: Pubkey,
    pub instance: u64,
    pub instance_address: Pubkey
}

#[event]
pub struct NewEntityMinted{
    pub world_instance: Pubkey,
    pub mint: Pubkey,
    pub entity: Pubkey
}