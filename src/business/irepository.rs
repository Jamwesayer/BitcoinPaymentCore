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
    fn check_payment_status(&self, label: &str) -> Result<PaymentDetails, String>;

    /// Returns Result<String, String>
    ///
    /// Refunds all the transactions done to
    ///
    /// # Arguments
    ///
    /// * `label` - A &str which will be used to identify the specific label for which the payments has been done
    fn refund(&self, label: &str) -> Result<String, String>;
}

use async_trait::async_trait;
#[async_trait]
pub trait ITransactionRepository {
    /// Follow transactions containing a certain label (identifier)
    ///
    /// # Arguments
    ///
    /// * `label` - A &str which will be used to identify the transactions that need to be followed
    async fn follow_transactions_for_label(&self, label: String);

    /// Find a specific transaction by it's transaction_id
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - A &str which species the to search transaction
    fn find_transaction_by_id(&self, transaction_id: &str);

    /// Returns all transactions for a certain identifier
    ///
    /// # Arguments
    ///
    /// * `label` - A &str to identify the incoming transactions
    fn get_all_transactions(&self, label: &str);

    fn save_transaction_to_database(&self);
}
