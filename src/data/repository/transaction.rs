use core::time::Duration;
use crate::data::idatasource::{ITransactionNetworkDataSource, IPaymentDatabaseDataSource, TransactionNetwork, PaymentDatabase};
use crate::business::irepository::{ITransactionRepository};

use tokio::time::sleep;
use tokio::task;

use std::thread;

pub struct TransactionRepository {
    transaction_network_datasource: Box<dyn ITransactionNetworkDataSource + Sync + Send>,
    transaction_database_datasource: Box<dyn IPaymentDatabaseDataSource + Sync + Send>
}

impl TransactionRepository {
    pub fn new(transaction_network_datasource: Box<dyn ITransactionNetworkDataSource + Sync + Send>, transaction_database_datasource: Box<dyn IPaymentDatabaseDataSource + Sync + Send>) -> Self {
        Self {
            transaction_network_datasource: transaction_network_datasource,
            transaction_database_datasource: transaction_database_datasource
        }
    }
}

use async_trait::async_trait;
#[async_trait]
impl ITransactionRepository for TransactionRepository {

    async fn follow_transactions_for_label(&self, label: String) {

        // let (tx, rx) = oneshot::channel::<i32>();

        // retrieve transaction of database linked to shop

        let expected_amount = 10.1;
        let mut total_amount = 0.0;

        let network_instance = dyn_clone::clone_box(&*self.transaction_network_datasource);

        task::spawn(async move {

            let mut skip = 0; // == amount of transaction for paymentwindows for the shopid

            loop {
                match network_instance.follow_transactions_for_label(&label, skip) {
                    Ok((amount, transactions)) => {
                        // save transactions to database with db ds
                        skip += transactions.len() as i32;
                        if total_amount >= expected_amount {
                            println!("Amount is correct {}", label);
                            // break;
                        } else {
                            if transactions.len() > 0 {
                                total_amount += amount;
                                println!("Amount: {}, for label: {}", amount, label);
                            }
                        }
                    },
                    Err(e) => println!("{:?}", e)
                }
            }
        });

        // thread::sleep(Duration::from_millis(20000));

    }

    fn find_transaction_by_id(&self, transaction_id: &str) {

    }
    fn get_all_transactions(&self, label: &str) {

    }
    fn save_transaction_to_database(&self) {

    }
}

impl Default for TransactionRepository {

    fn default() -> Self {
        Self {
            transaction_network_datasource: Box::new(TransactionNetwork::default()),
            transaction_database_datasource: Box::new(PaymentDatabase::default())
        }
    }
}
