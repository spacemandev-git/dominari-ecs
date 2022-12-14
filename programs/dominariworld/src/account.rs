use anchor_lang::prelude::*;
use std::collections::BTreeMap;

//use crate::state::*;

#[account]
pub struct WorldConfig{
    pub universe: Pubkey,
    pub components: u64,
}

#[account]
pub struct InstanceAuthority{
    pub instance: u64,
    pub authority: Pubkey
}

// PDA'd by Component ID which is just WorldSigner.Components + 1
#[account]
pub struct ComponentSchema{
    pub url: String,
}

#[account]
pub struct SystemRegistration{
    pub system: Pubkey,
    pub instance: u64,
    pub components: BTreeMap<Pubkey, bool>, //PDA of the Component Schema
}