use chrono::NaiveDateTime;

// PaymentRequestEntity
#[derive(Debug, PartialEq)]
pub struct TransactionEntity {
    amount: f64,
    transaction_id: String,
    origin_address: String,
    time_received: NaiveDateTime
}

impl TransactionEntity {
    pub fn new(amount: f64, transaction_id: String, origin_address: String, time_received: NaiveDateTime) -> Self {
        Self {
            amount: amount,
            transaction_id: transaction_id,
            origin_address: origin_address,
            time_received: time_received
        }
    }
}
