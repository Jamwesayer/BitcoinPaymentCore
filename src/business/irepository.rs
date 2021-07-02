use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

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
