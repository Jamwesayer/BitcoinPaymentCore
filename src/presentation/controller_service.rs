use crate::presentation::item::*;
use crate::business::usecase::transaction::TransactionUseCase;
use crate::business::usecase::payment::PaymentUseCase;

pub struct PaymentControllerService {
    payment_usecase: PaymentUseCase,
    transaction_usecase: TransactionUseCase
}

impl Default for PaymentControllerService {
    fn default() -> Self {
        Self {
            payment_usecase: PaymentUseCase::default(),
            transaction_usecase: TransactionUseCase::default()
        }
    }
}

impl PaymentControllerService {
    pub async fn create_payment_window(&self, payment_request_item: PaymentRequestItem) {
        let store_id = payment_request_item.get_store_id().clone();
        match self.payment_usecase.create_payment_window(payment_request_item) {
            Ok(generated_payment_request_item) => {
                self.transaction_usecase.follow_transaction(generated_payment_request_item.get_label().to_string(), *generated_payment_request_item.get_amount(), store_id).await;
            },
            Err(e) => println!("test")
        }
    }

    pub fn check_payment_status(&self, label: &str) -> Result<PaymentDetailsItem, String> {
        self.payment_usecase.check_payment_status(label)
    }

    pub fn refund(&self, label: &str)  -> Result<Vec<TransactionItem>, String> {
        self.payment_usecase.get_refund(label)
    }
}

// ---------------------------------- Transaction
pub struct TransactionControllerService {
    transaction_usecase: TransactionUseCase
}

impl Default for TransactionControllerService {
    fn default() -> Self {
        Self {
            transaction_usecase: TransactionUseCase::default()
        }
    }
}

impl TransactionControllerService {
    pub fn get_transaction_by_transaction_id(&self, transaction_id: &str) -> Result<TransactionItem, String> {
        self.transaction_usecase.get_transaction_by_transaction_id(transaction_id)
    }
}
