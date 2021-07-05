use crate::business::irepository::ITransactionRepository;
use crate::data::repository::transaction::TransactionRepository;
use crate::presentation::item::*;

pub struct TransactionUseCase {
    transaction_repository: Box<dyn ITransactionRepository>
}

impl TransactionUseCase {

    pub async fn follow_transaction(&self, label: String, amount: f64, store_id: i32) {
        self.transaction_repository.follow_transactions_for_label(label, amount, store_id).await;
    }

    pub fn get_transaction_by_transaction_id(&self, transaction_id: &str) -> Result<TransactionItem, String> {
        match self.transaction_repository.find_transaction_by_id(transaction_id) {
            Ok(transaction_model) => Ok(TransactionItem::map_to_presentation(transaction_model)),
            Err(e) => Err(e)
        }
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
