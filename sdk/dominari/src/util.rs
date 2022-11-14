use anchor_lang::AccountDeserialize;
use solana_client_wasm::WasmClient;
use anchor_lang::prelude::*;
use solana_sdk::commitment_config::CommitmentConfig;

pub async fn fetch_account<T: AccountDeserialize>(client: &WasmClient, pubkey: &Pubkey) -> Result<T> {
    let mut data:&[u8] = &client.get_account_with_commitment(pubkey, CommitmentConfig::confirmed()).await.unwrap().unwrap().data;
    let result = T::try_deserialize(&mut data).map_err(Into::into);
    return result;
}