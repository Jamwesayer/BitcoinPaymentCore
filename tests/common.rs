use payment_core::data::entity::transaction::TransactionEntity;
use payment_core::data::repository::transaction::TransactionRepository;
use payment_core::data::idatasource::ITransactionNetworkDataSource;
use payment_core::data::idatasource::ITransactionDatabaseDataSource;
use payment_core::data::repository::payment::PaymentRepository;
use payment_core::data::idatasource::IPaymentNetworkDataSource;
use payment_core::data::entity::payment::PaymentDetailsEntity;
use payment_core::data::entity::payment::PaymentRequestEntity;
use payment_core::data::entity::payment::GeneratedPaymentRequestEntity;
use payment_core::data::idatasource::IPaymentDatabaseDataSource;

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
    fn check_payment_window_status(&self, label: &str) -> std::result::Result<PaymentDetailsEntity, std::string::String> {
        if label == "ThisShouldWork" {
            Ok(PaymentDetailsEntity::new(label.to_string(),99.99, label.to_string() + "New", 1))
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
    fn get_payment_window_by_label(&self, label: &str) -> std::result::Result<(), std::string::String> {
        if label == "ThisShouldWork" {
            Ok(())
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
    fn suspend_payment_window(&self, label: &str) -> std::result::Result<(), std::string::String> {
        if label == "ThisShouldWork" {
            Ok(())
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
}

pub struct MockPaymentNetwork {

}

impl<'a> IPaymentNetworkDataSource for MockPaymentNetwork {
    fn send_refund(&self, _: &str) -> std::result::Result<std::vec::Vec<payment_core::data::entity::transaction::TransactionEntity>, std::string::String> { todo!() }
}

pub struct MockTransactionDatabase {

}

impl ITransactionDatabaseDataSource for MockTransactionDatabase {
    fn save_transaction(&self, _: &str, _: std::vec::Vec<payment_core::data::entity::transaction::TransactionEntity>) -> std::result::Result<(), std::string::String> { todo!() }
    fn get_transaction_by_transaction_id(&self, _: &str) -> std::result::Result<payment_core::data::entity::transaction::TransactionEntity, std::string::String> { todo!() }
    fn get_all_transactions(&self, _: &str) -> std::result::Result<std::vec::Vec<payment_core::data::entity::transaction::TransactionEntity>, std::string::String> { todo!() }
}

#[derive(Clone)]
pub struct MockTransactionNetwork {

}

impl ITransactionNetworkDataSource for MockTransactionNetwork {
    fn follow_transactions_for_label(&self, _: &str, _: i32) -> std::result::Result<(f64, std::vec::Vec<TransactionEntity>), String> { todo!() }
}

// --------------------------- FUNCTIONS
pub fn setup_correct_payment_repository() -> PaymentRepository {
    PaymentRepository::new(Box::new(MockPaymentNetwork {}), Box::new(MockPaymentDatabase {}))
}

pub fn setup_correct_transaction_repository() -> TransactionRepository {
    TransactionRepository::new(Box::new(MockTransactionNetwork {}),Box::new(MockTransactionDatabase {}))
}
