use dominari::solana_sdk::transaction::Transaction;
use solana_client_wasm::WasmClient;

pub fn send_tx_async(client: WasmClient, tx: Transaction) -> tokio::task::JoinHandle<()> { 
    tokio::spawn(async move {
        client.send_and_confirm_transaction(&tx).await.unwrap();
    })
}