use crate::data::entity::transaction::TransactionEntity;
use crate::business::model::Transaction;
use crate::data::idatasource::{ITransactionNetworkDataSource, ITransactionDatabaseDataSource, TransactionNetwork, TransactionDatabase};
use crate::business::irepository::{ITransactionRepository};

use tokio::task;
use async_trait::async_trait;

pub struct TransactionRepository {
    transaction_network_datasource: Box<dyn ITransactionNetworkDataSource + Sync + Send>,
    transaction_database_datasource: Box<dyn ITransactionDatabaseDataSource + Sync + Send>
}

impl TransactionRepository {
    pub fn new(transaction_network_datasource: Box<dyn ITransactionNetworkDataSource + Sync + Send>, transaction_database_datasource: Box<dyn ITransactionDatabaseDataSource + Sync + Send>) -> Self {
        Self {
            transaction_network_datasource: transaction_network_datasource,
            transaction_database_datasource: transaction_database_datasource
        }
    }
}

#[async_trait]
impl ITransactionRepository for TransactionRepository {

    async fn follow_transactions_for_label(&self, label: String, amount: f64, store_id: i32) {

        // retrieve transactions from database linked to shop

        let mut skip = 0;
        let expected_amount = amount;
        let mut total_amount = 0.0;

        let network_instance = dyn_clone::clone_box(&*self.transaction_network_datasource);
        let database_instance = dyn_clone::clone_box(&*self.transaction_database_datasource);

        task::spawn(async move {

            loop {
                match network_instance.follow_transactions_for_label(&label, skip) {
                    Ok((amount, transactions)) => {
                        // save transactions to database with db ds
                        // save_transaction_to_database()
                        skip += transactions.len() as i32;
                        if total_amount >= expected_amount {
                            println!("Amount is correct {}", label);
                            break;
                        } else {
                            if transactions.len() > 0 {
                                println!("{:?}", amount);
                                total_amount += amount;
                                database_instance.save_transaction(&label, transactions);
                                println!("Amount: {}, for label: {}", amount, label);
                            }
                        }
                    },
                    Err(e) => println!("{:?}", e)
                }
            }
        });

    }

    fn find_transaction_by_id(&self, transaction_id: &str) -> Result<Transaction, String> {
        match self.transaction_database_datasource.get_transaction_by_transaction_id(transaction_id) {
            Ok(transaction_entity) => Ok(transaction_entity.map_to_business()),
            Err(e) => Err(e.to_string())
        }
    }

    fn get_all_transactions(&self, label: &str) {

    }

    fn save_transaction_to_database(&self, label: &str, transactions: Vec<Transaction>) -> Result<(), String> {
        let mut transaction_entities: Vec<TransactionEntity> = Vec::new();
        for transaction in transactions {
            transaction_entities.push(TransactionEntity::map_to_entity(transaction));
        }
        self.transaction_database_datasource.save_transaction(label, transaction_entities)
    }
}

impl Default for TransactionRepository {

    fn default() -> Self {
        Self {
            transaction_network_datasource: Box::new(TransactionNetwork::default()),
            transaction_database_datasource: Box::new(TransactionDatabase::default())
        }
    }
}
