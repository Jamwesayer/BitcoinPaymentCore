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

    pub fn get_refund(&self, label: &str) -> Result<Vec<TransactionItem>, String> {
        match self.payment_repository.refund(label) {
            Ok(transactions) => {
                let transactions_cloned = transactions.clone();
                match self.transaction_repository.save_transaction_to_database(label, transactions) {
                    Ok(_) => {
                        let mut transaction_items: Vec<TransactionItem> = Vec::new();
                        for transaction in transactions_cloned {
                            transaction_items.push(TransactionItem::map_to_presentation(transaction));
                        }
                        Ok(transaction_items)
                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn suspend_payment_window(&self, label: &str) -> Result<(), String> {
        self.payment_repository.suspend_payment_window(label)
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
