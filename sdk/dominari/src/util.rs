use anchor_lang::AccountDeserialize;
use solana_client_wasm::WasmClient;
use anchor_lang::prelude::*;

pub async fn fetch_account<T: AccountDeserialize>(client: &WasmClient, pubkey: &Pubkey) -> Result<T> {
    let mut data:&[u8] = &client.get_account(pubkey).await.unwrap().data;
    let result: Result<T> = deserialize_account(&mut data).await;
    return result;
}

/**
 * Makes the assumption that the accounts returned are in the same order as the keys passed in
 * This is because the acocunts returned don't have the pubkey attached to them.
 */
pub async fn fetch_accounts<T: AccountDeserialize>(client: &WasmClient, pubkeys: &Vec<Pubkey>) -> Vec<(Pubkey,T)> {
    let accounts = &client.get_multiple_accounts(pubkeys).await.unwrap();
    let mut results = vec![];
    for (i, account) in accounts.iter().enumerate() {
        let result: Result<T> = deserialize_account(&account.as_ref().unwrap().data).await;
        results.push((pubkeys.get(i).unwrap().clone(), result.unwrap()));
    }
    return results;
}

pub async fn deserialize_account<T: AccountDeserialize>(mut data: &[u8]) -> Result<T> {
    let result = T::try_deserialize(&mut data).map_err(Into::into);
    return result;
}