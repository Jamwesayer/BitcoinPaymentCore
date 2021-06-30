use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

pub trait IPaymentRepository {
    /// Returns a GeneratedPaymentRequest
    ///
    /// # Arguments
    ///
    /// * `payment_request_model` - A PaymentRequest model
    fn create_payment_window(&self, payment_request_model: PaymentRequest) -> Result<GeneratedPaymentRequest, String>;

    /// Returns PaymentDetails
    ///
    /// # Arguments
    ///
    /// * `label` - A label which will be used to identify the address to check the payment status
    fn check_payment_status(&self, label: &str) -> Result<PaymentDetails, String>;

    /// Returns
    ///
    /// # Arguments
    ///
    /// * `label` - A label which will be used to identify the address to check the payment status
    fn refund(&self, label: &str) -> Result<String, String>;
}
