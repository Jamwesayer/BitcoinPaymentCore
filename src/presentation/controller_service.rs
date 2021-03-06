use crate::presentation::item::*;
use crate::business::usecase::transaction::TransactionUseCase;
use crate::business::usecase::payment::PaymentUseCase;
use crate::business::usecase::store::StoreUseCase;

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

    pub fn create_payment_window(&self, payment_request_item: &PaymentRequestItem) -> Result<GeneratedPaymentRequestItem, String> {
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

    pub fn suspend_payment_window(&self, payment_search_item: PaymentWindowSearchItem) -> Result<(String, Vec<TransactionItem>), String> {
        self.payment_usecase.suspend_payment_window(payment_search_item)
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
    pub fn get_all_transactions(&self, store_id: &i32) -> Result<Vec<TransactionItem>, String> {
        self.transaction_usecase.get_all_transactions(store_id)
    }
}

// ---------------------------------- Store
pub struct StoreControllerService {
    store_usecase: StoreUseCase
}

impl Default for StoreControllerService {
    fn default() -> Self {
        Self {
            store_usecase: StoreUseCase::default()
        }
    }
}

impl StoreControllerService {
    pub fn register(&self) -> Result<StoreItem, String> {
        let register = StoreItem::new("test".to_string(), "test".to_string(), "test".to_string());
        self.store_usecase.register_store(register)
    }

    pub fn login() {

    }
}