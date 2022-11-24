use anchor_lang::prelude::*;

use crate::component::MaxSize;


#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct RelevantComponentKeys {
    pub metadata: Pubkey,
    pub mapmeta: Pubkey,
    pub location: Pubkey,
    pub feature: Pubkey,
    pub owner: Pubkey,
    pub value: Pubkey,
    pub occupant: Pubkey,
    pub player_stats: Pubkey,
    pub last_used: Pubkey,
    pub rank: Pubkey,
    pub range: Pubkey,
    pub drop_table: Pubkey,
    pub uses: Pubkey,
    pub healing_power: Pubkey,
    pub health: Pubkey,
    pub damage: Pubkey,
    pub troop_class: Pubkey,
    pub active: Pubkey,
    pub cost: Pubkey,
}

impl MaxSize for RelevantComponentKeys {
    fn get_max_size() -> u64 {
        return 32*19;
    }
}