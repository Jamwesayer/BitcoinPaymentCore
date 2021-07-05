use crate::business::irepository::ITransactionRepository;
use crate::data::repository::transaction::TransactionRepository;
use crate::presentation::item::*;

pub struct TransactionUseCase {
    transaction_repository: Box<dyn ITransactionRepository>
}

impl TransactionUseCase {

    pub async fn follow_transaction(&self, label: String, amount: f64) {
        self.transaction_repository.follow_transactions_for_label(label).await;
    }

    pub fn new(transaction_repository: Box<dyn ITransactionRepository>) -> Self {
        Self {
            transaction_repository: transaction_repository
        }
    }

}

impl Default for TransactionUseCase {

    fn default() -> Self {
        Self {
            transaction_repository: Box::new(TransactionRepository::default())
        }
    }

}
