use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SerializedComponent{
    pub component_key: Pubkey,
    pub component_update_authority: Pubkey, // PDA of ComponentKey "update_authority"
    pub schema_url: String, // Max 256 bytes ?
    pub max_size: u64,
    pub data: Vec<u8>,
}