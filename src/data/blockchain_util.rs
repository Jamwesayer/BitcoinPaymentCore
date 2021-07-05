extern crate bitcoincore_rpc;
use crate::data::entity::transaction::TransactionEntity;
use bitcoincore_rpc::{bitcoin, Auth, Client, RpcApi};
use bitcoin::{Address, Amount};
use std::str::FromStr;
use std::convert::TryInto;

fn open_connection() -> bitcoincore_rpc::Client {
    return Client::new("http://localhost:18443".to_string(),
                          Auth::UserPass("test".to_string(),
                                         "test".to_string())).unwrap();
}

/// Returns Result<_, String>
///
/// Imports a public key to the systems, this allows it to check for transactions and handle transactions connected to the address accordingly
///
/// # Arguments
///
/// * `address` - A &str which represents the public key of the address that needs to be connected to the service
pub fn import_address(address: &str) -> Result<(), String> {
    match Address::from_str(address) {
        Ok(add) => {
            match open_connection().import_address(&add, None, Some(false)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }

}

/// Returns Result<String, String>
///
/// It gathers al transactions with the given label, then sends transaction to the found addresses with the according amount minus the fee
///
/// # Arguments
///
/// * `label` - A &str which will be used to identify the transactions done with the "transaction_id"
pub fn refund(label: &str) -> Result<Vec::<TransactionEntity>, String> {
    match get_all_transactions_for_address_by_label(label, None) {
        Ok(transactions) => {
            let mut transaction_entities: Vec<TransactionEntity> = Vec::new();
            for transaction in transactions {
                transaction_entities.push(
                    TransactionEntity::new(
                        transaction.detail.amount.as_btc(),
                        transaction.info.txid.to_string(),
                        transaction.detail.address.as_ref().unwrap().to_string(),
                        2,
                        chrono::NaiveDateTime::from_timestamp(0, transaction.info.timereceived.try_into().unwrap())
                    )
                );
                println!("{:?}", transaction.detail.address);
                println!("{:?}", transaction.detail.amount);
                send_transaction_to_address(&transaction.detail.address.unwrap(), Amount::from_sat(transaction.detail.amount.as_sat() as u64));
            }
            Ok(transaction_entities)
        },
        Err(e) => Err(e.to_string())
    }
}

/// Returns Result<Vec<bitcoincore_rpc_json::ListTransactionResult>, String>
///
/// It retrieves all address which are done to the given label
///
/// # Arguments
///
/// * `label` - A &str which will be used to identify the transactions with the "transaction_id"
fn get_all_transactions_for_address_by_label(label: &str, skip: Option<usize>) -> Result<Vec<bitcoincore_rpc_json::ListTransactionResult>, String> {
    match open_connection().list_transactions(Some(label), None, skip, None) {
        Ok(transactions) => {
            let mut filtered_transactions: Vec<bitcoincore_rpc_json::ListTransactionResult> = Vec::new();
            for transaction in transactions {
                if transaction.detail.label.eq(&Some(label.to_string())) {
                    filtered_transactions.push(transaction);
                }
            }
            Ok(filtered_transactions)
        },
        Err(e) => Err(e.to_string())
    }
}

pub fn get_all_transactions_for_address_by_label_with_total(label: &str, skip: i32) -> Result<(f64, Vec<TransactionEntity>), String> {
    match get_all_transactions_for_address_by_label(label, Some(skip.try_into().unwrap())) {
        Ok(transactions) => {
            let mut transaction_entities: Vec<TransactionEntity> = Vec::new();
            let mut amount = 0.0;
            for transaction in &transactions {
                transaction_entities.push(
                    TransactionEntity::new(
                        transaction.detail.amount.as_btc(),
                        transaction.info.txid.to_string(),
                        transaction.detail.address.as_ref().unwrap().to_string(),
                        1,
                        chrono::NaiveDateTime::from_timestamp(0, transaction.info.timereceived.try_into().unwrap())
                    )
                );
                println!("{:?}", transaction.detail.amount.as_btc());
                amount += transaction.detail.amount.as_btc();
            };
            Ok((amount, transaction_entities))
        },
        Err(e) => Err(e.to_string())
    }
}

/// It sends a transaction accordngly to the given address with the given amount
///
/// # Arguments
///
/// * `address` - A &Address that represents the receipient
/// * `amount` - An Amount which indicates the required sats to send
fn send_transaction_to_address(address: &Address, amount: Amount) {
    open_connection().send_to_address(address, amount, None, None, None, None, None, None);
}
