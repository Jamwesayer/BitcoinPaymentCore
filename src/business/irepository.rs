use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

pub trait IPaymentRepository {
    /// Returns a GeneratedPaymentRequest
    ///
    /// # Arguments
    ///
    /// * `payment_request_model` - A PaymentRequest model
    fn create_payment_window(&self, payment_request_model: PaymentRequest) -> GeneratedPaymentRequest;
}
