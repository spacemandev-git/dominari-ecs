use anchor_lang::prelude::*;

#[event]
pub struct NewWorldInstance {
    pub world_instance: Pubkey,
    pub instance_authority: Pubkey
}

#[event]
pub struct NewComponentRegistered {
    pub component: Pubkey,
    pub schema: String
}

#[event]
pub struct NewSystemRegistration {
    pub world_instance: Pubkey,
    pub component: Pubkey,
    pub system: Pubkey,
    pub system_registration: Pubkey
}