use payment_core::data::entity::transaction::*;
use payment_core::data::repository::transaction::TransactionRepository;
use payment_core::data::idatasource::ITransactionNetworkDataSource;
use payment_core::data::idatasource::ITransactionDatabaseDataSource;
use payment_core::data::repository::payment::PaymentRepository;
use payment_core::data::idatasource::IPaymentNetworkDataSource;
use payment_core::data::entity::payment::*;
use payment_core::data::idatasource::IPaymentDatabaseDataSource;

use chrono::naive::*;

#[derive(Clone)]
pub struct MockPaymentDatabase {

}

impl IPaymentDatabaseDataSource for MockPaymentDatabase {

    fn insert_payment_window(&self, request_entity: &PaymentRequestEntity) -> std::result::Result<GeneratedPaymentRequestEntity, std::string::String> {
        if request_entity.get_label().eq("ThisShouldWork") {
            Ok(GeneratedPaymentRequestEntity::new(request_entity.get_label().to_string(), *request_entity.get_amount(), request_entity.get_label().to_string() + "New", *request_entity.get_store_id()))
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
    fn check_payment_window_status(&self, payment_search_entity: PaymentWindowSearchEntity) -> std::result::Result<PaymentDetailsEntity, std::string::String> {
        if payment_search_entity.get_label() == "ThisShouldWork" {
            Ok(PaymentDetailsEntity::new(payment_search_entity.get_label().to_string(),99.99, payment_search_entity.get_label().to_string() + "New", 1))
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
    fn get_payment_window_by_label(&self, payment_search_entity: PaymentWindowSearchEntity) -> std::result::Result<(), std::string::String> {
        if payment_search_entity.get_label() == "ThisShouldWork" {
            Ok(())
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
    fn suspend_payment_window(&self, payment_search_entity: PaymentWindowSearchEntity) -> std::result::Result<(), std::string::String> {
        if payment_search_entity.get_label() == "ThisShouldWork" {
            Ok(())
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
}

#[derive(Clone)]
pub struct MockPaymentNetwork {

}

impl<'a> IPaymentNetworkDataSource for MockPaymentNetwork {
    fn send_refund(&self, label: &str) -> Result<Vec<TransactionEntity>, String> {
        let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        Ok(vec![TransactionEntity::new(10.0, "hash".to_string(), "origin".to_string(), 1, dt)])
    }
    fn create_payment_window(&self, label: &str) -> Result<String, String> { Ok(label.to_string() + "New") }
}

#[derive(Clone)]
pub struct MockTransactionDatabase {

}

impl ITransactionDatabaseDataSource for MockTransactionDatabase {
    fn save_transaction(&self, label: &str, store_id: &i32, transaction_entities: Vec::<TransactionEntity>) -> Result<(), String> {
        Ok(())
    }
    fn get_transaction_by_transaction_id(&self, transaction_id: &str) -> Result<TransactionEntity, String>{
        let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        Ok(TransactionEntity::new(10.0, "hash".to_string(), "origin".to_string(), 1, dt))
    }
    fn get_total_transactions_by_store_id(&self, store_id: &i32) -> Result<i64, String> {
        Ok(10)
    }
    fn get_all_transactions(&self, label: &str) -> Result<Vec<TransactionEntity>, String> {
        Err("Error".to_string())
    }
}

#[derive(Clone)]
pub struct MockTransactionNetwork {

}

impl ITransactionNetworkDataSource for MockTransactionNetwork {
    fn follow_transactions_for_label(&self, label: &str, _: i32) -> std::result::Result<(f64, std::vec::Vec<TransactionEntity>), String> {
        let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        Ok((10.8, vec![TransactionEntity::new(10.0, "hash".to_string(), "origin".to_string(), 1, dt)]))
    }
}

// --------------------------- FUNCTIONS
pub fn setup_correct_payment_repository() -> PaymentRepository {
    PaymentRepository::new(Box::new(MockPaymentNetwork {}), Box::new(MockPaymentDatabase {}))
}

pub fn setup_correct_transaction_repository() -> TransactionRepository {
    TransactionRepository::new(Box::new(MockTransactionNetwork {}),Box::new(MockTransactionDatabase {}))
}
