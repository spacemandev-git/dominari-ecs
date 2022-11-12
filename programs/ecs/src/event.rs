use anchor_lang::prelude::*;

use crate::state::*;

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

#[event]
pub struct NewComponentAdded{
    pub entity: Pubkey,
    pub components: Vec<SerializedComponent>
}

#[event]
pub struct ComponentRemoved{
    pub entity: Pubkey,
    pub component: SerializedComponent
}

#[event]
pub struct ComponentModified{
    pub entity: Pubkey,
    pub component: Pubkey
}