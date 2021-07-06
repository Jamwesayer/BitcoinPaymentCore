use crate::presentation::controller_service::PaymentControllerService;
use crate::presentation::item::*;

pub struct PaymentController {
    payment_controller_service: PaymentControllerService
}

impl Default for PaymentController {
    fn default() -> Self {
        Self {
            payment_controller_service: PaymentControllerService::default()
        }
    }
}

impl PaymentController {
    pub async fn create_payment_window(&self, payment_request_item: PaymentRequestItem) {
        let store_id = payment_request_item.get_store_id().clone();
        match self.payment_controller_service.create_payment_window(payment_request_item) {
            Ok(generated_payment_request_item) => {
                println!("Address: {:?}", generated_payment_request_item.get_address());
                self.payment_controller_service.follow_transaction_for_label(generated_payment_request_item, store_id).await
            },
            Err(e) => println!("{:?}", e)
        }
    }

    pub fn refund(&self, label: &str) {
        self.payment_controller_service.refund(label);
    }
}
