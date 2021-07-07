use dyn_clone::DynClone;
use crate::data::blockchain_util as blockchain;
use crate::data::database_util as database;
use crate::data::entity::payment::*;
use crate::data::entity::transaction::*;

//------------------------------------------------------------------------------------------------- Payment
pub trait IPaymentDatabaseDataSource {
    fn insert_payment_window(&self, payment_window_entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, String>;
    fn check_payment_window_status(&self, payment_search_entity: PaymentWindowSearchEntity) -> Result<PaymentDetailsEntity, String>;
    fn get_payment_window_by_label(&self, payment_search_entity: PaymentWindowSearchEntity) -> Result<(), String>;
    fn suspend_payment_window(&self, payment_search_entity: PaymentWindowSearchEntity) -> Result<(), String>;
}

pub struct PaymentDatabase {}

impl Default for PaymentDatabase {
    fn default() -> Self {
        Self{}
    }
}

impl<'a> IPaymentDatabaseDataSource for PaymentDatabase {
    fn insert_payment_window(&self, payment_window_entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, String> {
        match database::insert_payment_window(payment_window_entity) {
            Ok(generated_payment_request_entity) => Ok(generated_payment_request_entity),
            Err(e) => Err(e.to_string())
        }
    }

    fn check_payment_window_status(&self, payment_search_entity: PaymentWindowSearchEntity) -> Result<PaymentDetailsEntity, String>{
        if let Ok(payment_window) = database::get_payment_window_by_label(payment_search_entity.get_label(), payment_search_entity.get_store_id()) {
            match database::get_store_wallet_by_id(&payment_window.store_id) {
                Ok(store) => {
                    Ok(PaymentDetailsEntity::new(payment_window.label, payment_window.amount, store.wallet_address, payment_window.status_id))
                },
                Err(e) => Err(e.to_string())
            }

        } else {
            Err("Error".to_string())
        }
    }
    fn get_payment_window_by_label(&self, payment_search_entity: PaymentWindowSearchEntity) -> Result<(), String> {
        match database::get_payment_window_by_label(payment_search_entity.get_label(), payment_search_entity.get_store_id()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
    fn suspend_payment_window(&self, payment_search_entity: PaymentWindowSearchEntity) -> Result<(), String> {
        match database::suspend_payment_window(payment_search_entity.get_label(), payment_search_entity.get_store_id()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}

pub trait IPaymentNetworkDataSource {
    fn create_payment_window(&self, label: &str) -> Result<String, String>;
    fn send_refund(&self, label: &str) -> Result<Vec::<TransactionEntity>, String>;
}

pub struct PaymentNetwork {}

impl Default for PaymentNetwork {
    fn default() -> Self {
        Self{}
    }
}

impl IPaymentNetworkDataSource for PaymentNetwork {

    fn create_payment_window(&self, label: &str) -> Result<String, String> {
        match blockchain::create_receiving_address(label) {
            Ok(address_string) => Ok(address_string),
            Err(e) => Err(e.to_string())
        }
    }

    fn send_refund(&self, label: &str) -> Result<Vec::<TransactionEntity>, String> {
        blockchain::refund(label)
    }
}

//------------------------------------------------------------------------------------------------- Transaction
pub trait ITransactionDatabaseDataSource: DynClone {
    fn save_transaction(&self, label: &str, store_id: &i32, transaction_entities: Vec::<TransactionEntity>) -> Result<(), String>;
    fn get_transaction_by_transaction_id(&self, transaction_id: &str) -> Result<TransactionEntity, String>;
    fn get_total_transactions_by_store_id(&self, store_id: &i32) -> Result<i64, String>;
    fn get_all_transactions(&self, label: &str) -> Result<Vec<TransactionEntity>, String>;
}

#[derive(Clone)]
pub struct TransactionDatabase {}

impl Default for TransactionDatabase {
    fn default() -> Self {
        Self{}
    }
}

impl ITransactionDatabaseDataSource for TransactionDatabase {
    fn save_transaction(&self, label: &str, store_id: &i32, transaction_entities: Vec<TransactionEntity>) -> Result<(), String> {
        match database::insert_transactions(label, store_id, transaction_entities) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
    fn get_transaction_by_transaction_id(&self, transaction_id: &str) -> Result<TransactionEntity, String> {
        match database::get_transaction_by_transaction_id(transaction_id) {
            Ok(transaction) => Ok(TransactionEntity::new(transaction.amount, transaction.hash, transaction.from_address, transaction.transaction_type_id, transaction.date)),
            Err(e) => Err(e.to_string())
        }
    }

    fn get_total_transactions_by_store_id(&self, store_id: &i32) -> Result<i64, String> {
        match database::get_amount_of_transactions_for_shop(store_id) {
            Ok(amount) => Ok(amount),
            Err(e) => Err(e.to_string())
        }
    }

    fn get_all_transactions(&self, label: &str) -> Result<Vec<TransactionEntity>, String> {
        Err("test".to_string())
    }
}

pub trait ITransactionNetworkDataSource: DynClone  {
    fn follow_transactions_for_label(&self, label: &str, skip: i32) -> Result<(f64, Vec<TransactionEntity>), String>;
}

#[derive(Clone)]
pub struct TransactionNetwork {}

impl Default for TransactionNetwork {
    fn default() -> Self {
        Self{}
    }
}

impl ITransactionNetworkDataSource for TransactionNetwork {
    fn follow_transactions_for_label(&self, label: &str, skip: i32) -> Result<(f64, Vec<TransactionEntity>), String>{
        blockchain::get_all_transactions_for_address_by_label_with_total(&label, skip)
    }
}
