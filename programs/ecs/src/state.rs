use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SerializedComponent{
    pub component_key: Pubkey, // PDA from World Program
    pub component_update_authority: Pubkey, // World Program usually
    pub schema_url: String, // Max 256 bytes ?
    pub max_size: usize,
    pub data: Vec<u8>,
}