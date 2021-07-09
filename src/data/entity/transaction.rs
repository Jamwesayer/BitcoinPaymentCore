use crate::business::model::Transaction;
use chrono::NaiveDateTime;

// TransactionEntity
#[derive(Debug, PartialEq)]
pub struct TransactionEntity {
    amount: f64,
    transaction_id: String,
    origin_address: String,
    transaction_type: i32,
    time_received: NaiveDateTime,
    transaction_status_id: i32,
}

impl TransactionEntity {
    pub fn new(amount: f64, transaction_id: String, origin_address: String, transaction_type: i32, time_received: NaiveDateTime, transaction_status_id: i32) -> Self {
        Self {
            amount: amount,
            transaction_id: transaction_id,
            origin_address: origin_address,
            transaction_type: transaction_type,
            time_received: time_received,
            transaction_status_id: transaction_status_id
        }
    }

    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }

    pub fn get_transaction_id(&self) -> &str {
        self.transaction_id.as_str()
    }
    pub fn get_time_received(&self) -> &NaiveDateTime {
        &self.time_received
    }
    pub fn get_transaction_type(&self) -> &i32 {
        &self.transaction_type
    }
    pub fn get_origin_address(&self) -> &str {
        self.origin_address.as_str()
    }
    pub fn get_transaction_status(&self) -> &i32 {
        &self.transaction_status_id
    }

    pub fn map_to_entity(model: Transaction) -> Self {
        TransactionEntity {
            amount: *model.get_amount(),
            transaction_id: model.get_transaction_id().to_string(),
            origin_address: model.get_origin_address().to_string(),
            transaction_type: *model.get_transaction_type(),
            time_received: *model.get_time_received(),
            transaction_status_id: *model.get_transaction_status()
        }
    }

    pub fn map_to_business(&self) -> Transaction {
        Transaction::new(*self.get_amount(), self.get_transaction_id().to_string(), self.get_origin_address().to_string(), *self.get_transaction_type(), *self.get_time_received(), *self.get_transaction_status())
    }
}
