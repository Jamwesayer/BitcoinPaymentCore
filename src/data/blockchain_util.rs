extern crate bitcoincore_rpc;
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};
use bitcoin::{Address, Amount, SignedAmount};
use std::str::FromStr;

fn open_connection() -> bitcoincore_rpc::Client {
    return Client::new("http://localhost:18443".to_string(),
                          Auth::UserPass("test".to_string(),
                                         "test".to_string())).unwrap();
}

pub fn create_wallet(address: &str, wallet: &str) -> Result<(), String> {
    match Address::from_str(address) {
        Ok(add) => {
            match open_connection().import_address(&add, Some(wallet), Some(false)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }

}

pub fn refund(label: &str) -> Result<String, String> {
    match get_all_transactions_for_address_by_label(label) {
        Ok(transactions) => {
            for transaction in transactions {
                println!("{:?}", transaction.detail.address);
                println!("{:?}", transaction.detail.amount);
                send_transaction_to_address(&transaction.detail.address.unwrap(), Amount::from_sat(transaction.detail.amount.as_sat() as u64));
            }
            Ok("Good".to_string())
        },
        Err(e) => Err(e.to_string())
    }
}

pub fn get_all_transactions_for_address_by_label(label: &str) -> Result<Vec<bitcoincore_rpc_json::ListTransactionResult>, String> {
    match open_connection().list_transactions(Some(label), None, None, None) {
        Ok(transactions) => Ok(transactions),
        Err(e) => Err(e.to_string())
    }
}

pub fn send_transaction_to_address(address: &Address, amount: Amount) {
    open_connection().send_to_address(address, amount, None, None, None, None, None, None);
}
