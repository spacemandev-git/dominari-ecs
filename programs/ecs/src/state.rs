use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SerializedComponent{
    pub component_key: Pubkey, // PDA from World Program
    pub world: Pubkey, // Update Authority
    pub max_size: usize,
    pub data: Vec<u8>,
}