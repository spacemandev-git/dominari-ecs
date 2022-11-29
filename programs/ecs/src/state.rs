use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SerializedComponent{
    pub component_key: Pubkey,
    pub max_size: u64,
    pub data: Vec<u8>,
}