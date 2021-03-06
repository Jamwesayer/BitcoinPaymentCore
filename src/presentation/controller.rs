use crate::presentation::controller_service::*;
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
        match self.payment_controller_service.create_payment_window(&payment_request_item) {
            Ok(generated_payment_request_item) => {
                println!("Address: {:?}", generated_payment_request_item.get_address());
                generated_payment_request_item.generate_qr_code_image();
                self.payment_controller_service.follow_transaction_for_label(generated_payment_request_item, *payment_request_item.get_store_id()).await
            },
            Err(e) => println!("{:?}", e)
        }
    }

    pub fn check_payment_status(&self, payment_search_item: PaymentWindowSearchItem) {
        match self.payment_controller_service.check_payment_status(payment_search_item) {
            Ok(payment_details_item) => println!("{:?}", payment_details_item),
            Err(e) => println!("{:?}", e)
        }
    }

    fn refund(&self, payment_search_item: PaymentWindowSearchItem) {
        self.payment_controller_service.refund(payment_search_item);
    }

    pub fn suspend_payment_window(&self, payment_search_item: PaymentWindowSearchItem) {
        match self.payment_controller_service.suspend_payment_window(payment_search_item) {
            Ok(success) => println!("{:?}", success),
            Err(e) => println!("{:?}", e)
        }
    }
}

// -----------------------------------------------Transactions
pub struct TransactionController {
    transaction_controller_service: TransactionControllerService
}

impl Default for TransactionController {
    fn default() -> Self {
        Self {
            transaction_controller_service: TransactionControllerService::default()
        }
    }
}

impl TransactionController {
    pub fn get_all_transactions(&self, store_id: &i32) {
        match self.transaction_controller_service.get_all_transactions(store_id) {
            Ok(transactions) => {
                println!("{:?}", transactions);
            },
            Err(e) => {println!("{:?}", e);}
        }
    }
}

// pub struct StoreController {
//     store_controller_service:
// }
