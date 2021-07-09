use crate::business::irepository::{IPaymentRepository, ITransactionRepository};
use crate::presentation::item::*;
use crate::data::repository::payment::{PaymentRepository};
use crate::data::repository::transaction::{TransactionRepository};

pub struct PaymentUseCase {
    payment_repository: Box<dyn IPaymentRepository>,
    transaction_repository: Box<dyn ITransactionRepository>
}

impl PaymentUseCase {
    pub fn create_payment_window(&self, payment_request_item: PaymentRequestItem) -> Result<GeneratedPaymentRequestItem, String> {
        let generated_payment_request_model = self.payment_repository.create_payment_window(payment_request_item.map_to_business())?;
        Ok(GeneratedPaymentRequestItem::map_to_presentation(generated_payment_request_model))
    }

    pub fn check_payment_status(&self, payment_search_item: PaymentWindowSearchItem) -> Result<PaymentDetailsItem, String> {
        let payment_details_model = self.payment_repository.check_payment_status(payment_search_item.map_to_business())?;
        Ok(PaymentDetailsItem::map_to_presentation(payment_details_model))
    }

    pub fn get_refund(&self, payment_search_item: PaymentWindowSearchItem) -> Result<Vec<TransactionItem>, String> {
        let cloned_payment_search_item = payment_search_item.clone();
        let transactions = self.payment_repository.refund(payment_search_item.map_to_business())?;

        let transactions_cloned = transactions.clone();
        self.transaction_repository.save_transaction_to_database(cloned_payment_search_item.get_label(), cloned_payment_search_item.get_store_id(), transactions)?;

        let mut transaction_items: Vec<TransactionItem> = Vec::new();
        for transaction in transactions_cloned {
            transaction_items.push(TransactionItem::map_to_presentation(transaction));
        }
        Ok(transaction_items)
    }

    pub fn suspend_payment_window(&self, payment_search_item: PaymentWindowSearchItem) -> Result<(String, Vec<TransactionItem>), String> {
        let payment_search_item_cloned = payment_search_item.clone();
        match self.payment_repository.suspend_payment_window(payment_search_item.map_to_business()) {
            Ok(_) => {
                let transactions = self.get_refund(payment_search_item_cloned)?;
                Ok(("De betaling is geanulleerd".to_string(), transactions))
            },
            Err(e) => Err(e)
        }
    }

    pub fn new(payment_repository: Box<dyn IPaymentRepository>, transaction_repository: Box<dyn ITransactionRepository>) -> Self {
        Self {
            payment_repository: payment_repository,
            transaction_repository: transaction_repository
        }
    }

}

impl Default for PaymentUseCase {

    fn default() -> Self {
        Self {
            payment_repository: Box::new(PaymentRepository::default()),
            transaction_repository: Box::new(TransactionRepository::default())
        }
    }

}
