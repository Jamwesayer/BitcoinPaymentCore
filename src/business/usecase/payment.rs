use crate::business::irepository::IPaymentRepository;
use crate::presentation::item::{GeneratedPaymentRequestItem, PaymentDetailsItem, PaymentRequestItem};

pub struct PaymentUseCase<'a> {
    payment_repository: &'a dyn IPaymentRepository
}

impl<'a> PaymentUseCase<'a> {
    pub fn create_payment_window(&self, payment_request_item: PaymentRequestItem) -> Result<GeneratedPaymentRequestItem, String> {
        match self.payment_repository.create_payment_window(payment_request_item.map_to_business()) {
            Ok(generated_payment_request_model) => Ok(GeneratedPaymentRequestItem::map_to_presentation(generated_payment_request_model)),
            Err(e) => Err(e)
        }
    }

    pub fn check_payment_status(&self, label: &str) -> Result<PaymentDetailsItem, String> {
        match self.payment_repository.check_payment_status(label) {
            Ok(payment_details_model) => Ok(PaymentDetailsItem::map_to_presentation(payment_details_model)),
            Err(e) => Err(e)
        }
    }

    pub fn get_refund(&self, label: &str) -> Result<String, String> {
        self.payment_repository.refund(label)
    }

    pub fn new(_payment_repository: &'a dyn IPaymentRepository) -> Self {
        Self {
            payment_repository: _payment_repository
        }
    }

}

// fn new() -> self {
//     return payment {
//         paymentRepository: PaymentRepository {}
//     }
// }
