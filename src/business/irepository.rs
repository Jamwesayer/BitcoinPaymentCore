use crate::business::model::*;

pub trait IPaymentRepository {
    /// Returns Result<GeneratedPaymentRequest, String>
    ///
    /// Creates a paymentwindow which will allow customers to pay and keep track of the payments using an unique identifier (transaction_id)
    ///
    /// # Arguments
    ///
    /// * `payment_request_model` - A PaymentRequest model
    fn create_payment_window(&self, payment_request_model: PaymentRequest) -> Result<GeneratedPaymentRequest, String>;

    /// Returns Result<PaymentDetails, String>
    ///
    /// Check the status of a certain payment window
    ///
    /// # Arguments
    ///
    /// * `label` - A &str which will be used to identify the payment window
    fn check_payment_status(&self, payment_search_model: PaymentWindowSearch) -> Result<PaymentDetails, String>;

    /// Returns Result<String, String>
    ///
    /// Refunds all the transactions done to
    ///
    /// # Arguments
    ///
    /// * `label` - A &str which will be used to identify the specific label for which the payments has been done
    fn refund(&self, payment_search_model: PaymentWindowSearch) -> Result<Vec<Transaction>, String>;

    fn suspend_payment_window(&self, payment_search_model: PaymentWindowSearch) -> Result<(), String>;
}

use async_trait::async_trait;
#[async_trait]
pub trait ITransactionRepository {
    /// Follow transactions containing a certain label (identifier)
    ///
    /// # Arguments
    ///
    /// * `label` - A &str which will be used to identify the transactions that need to be followed
    async fn follow_transactions_for_label(&self, label: String, amount: f64, store_id: i32);

    /// Find a specific transaction by it's transaction_id
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - A &str which species the to search transaction
    fn find_transaction_by_id(&self, transaction_id: &str) -> Result<Transaction, String>;

    /// Returns all transactions for a certain identifier
    ///
    /// # Arguments
    ///
    /// * `label` - A &str to identify the incoming transactions
    fn get_all_transactions(&self, store_id: &i32) -> Result<Vec<Transaction>, String>;

    fn save_transaction_to_database(&self, label: &str, store_id: &i32, transactions: &Vec<Transaction>) -> Result<(), String>;
}
