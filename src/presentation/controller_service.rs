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
    pub fn create_payment_window(&self, payment_request_item: PaymentRequestItem) -> Result<GeneratedPaymentRequestItem, String> {
        self.payment_usecase.create_payment_window(payment_request_item)
    }

    pub async fn follow_transaction_for_label(&self, generated_payment_request_item: GeneratedPaymentRequestItem, store_id: i32) {
        self.transaction_usecase.follow_transaction(generated_payment_request_item.get_label().to_string(), *generated_payment_request_item.get_amount(), store_id).await;
    }

    pub fn check_payment_status(&self, payment_search_item: PaymentWindowSearchItem) -> Result<PaymentDetailsItem, String> {
        self.payment_usecase.check_payment_status(payment_search_item)
    }

    pub fn refund(&self, payment_search_item: PaymentWindowSearchItem)  -> Result<Vec<TransactionItem>, String> {
        self.payment_usecase.get_refund(payment_search_item)
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
