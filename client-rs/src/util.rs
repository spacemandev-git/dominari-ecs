use dominari::solana_sdk::transaction::Transaction;
use solana_client_wasm::WasmClient;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};

use crate::RPC_URL;

pub fn send_tx_async(client: WasmClient, tx: Transaction) -> tokio::task::JoinHandle<()> { 
    tokio::spawn(async move {
        client.send_and_confirm_transaction(&tx).await.unwrap();
    })
}

pub fn send_tx_skip_preflight(tx: Transaction) {
    let rpc = RpcClient::new(RPC_URL);
    rpc.send_transaction_with_config(&tx, RpcSendTransactionConfig { skip_preflight: true, ..Default::default() }).unwrap();
}