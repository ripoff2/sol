use solana_client::nonblocking::rpc_client::RpcClient as Client;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::TransactionDetails;
use crate::utils::{get_network};
use solana_transaction_status::UiTransactionEncoding::Base64;

pub async fn handler(rpc_url: String, slot: crate::Slot) {
    // Build RPC Client
    let client = Client::new(get_network(&rpc_url));

    let config = RpcBlockConfig {
        encoding: Option::from(Base64),
        transaction_details: Option::from(TransactionDetails::Full),
        rewards: Option::from(false),
        commitment: Option::from(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Option::from(0),
    };

    // Fetch slot
    let fetched_slot = client.get_block_with_config(slot.slot, config).await.unwrap();
    let time_date = chrono::NaiveDateTime::from_timestamp_opt(fetched_slot.block_time.unwrap(), 0).unwrap();
    let txs_s = fetched_slot.transactions.clone().unwrap().iter().filter(|tx| tx.meta.as_ref().unwrap().err.is_none()).count();
    let txs_f = fetched_slot.transactions.clone().unwrap().iter().filter(|tx| tx.meta.as_ref().unwrap().err.is_some()).count();
    println!("Slot: {:?}", slot.slot);
    println!("Time: {:?} UTC - {:?}", time_date, fetched_slot.block_time.unwrap());
    print!("Transactions: Total: {:?}, Successful: {:?}, Failed: {:?}", fetched_slot.transactions.unwrap().len(), txs_s, txs_f);

    let _file_name = format!("slot_{}.json", slot.slot);
    // // Display slot
    // println!();
    // println!("Saved slot {} info to {}", slot.slot, file_name);
    // println!();
}