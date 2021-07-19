use chrono::NaiveDateTime;
use crate::business::model::*;

#[derive(Debug, PartialEq)]
pub struct PaymentRequestItem {
    label: String,
    amount: f64,
    store_id: i32
}

impl<'a> PaymentRequestItem {
    pub fn new(label: String, amount: f64, store_id: i32) -> Self {
        PaymentRequestItem {
            label: label,
            amount: amount,
            store_id: store_id
        }
    }
    pub fn get_label(&self) -> &str {
        self.label.as_str()
    }
    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }
    pub fn get_store_id(&self) -> &i32 {
        &self.store_id
    }

    pub fn map_to_business(&self) -> PaymentRequest {
        PaymentRequest::new(self.get_label().to_string(), *self.get_amount(), *self.get_store_id())
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct PaymentWindowSearchItem {
    label: String,
    store_id: i32
}

impl PaymentWindowSearchItem {
    pub fn new(label: String, store_id: i32) -> Self {
        PaymentWindowSearchItem {
            label: label,
            store_id: store_id
        }
    }
    pub fn get_label(&self) -> &str {
        self.label.as_str()
    }
    pub fn get_store_id(&self) -> &i32 {
        &self.store_id
    }
    pub fn map_to_business(&self) -> PaymentWindowSearch {
        PaymentWindowSearch::new(self.get_label().to_string(), *self.get_store_id())
    }
}

#[derive(Debug, PartialEq)]
pub struct GeneratedPaymentRequestItem {
    label: String,
    amount: f64,
    address: String
}

impl<'a> GeneratedPaymentRequestItem {
    pub fn new(label: String, amount: f64, address: String) -> Self {
        GeneratedPaymentRequestItem {
            label: label,
            amount: amount,
            address: address
        }
    }
    pub fn get_label(&self) -> &str {
        self.label.as_str()
    }
    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }
    pub fn get_address(&self) -> &str {
        &self.address.as_str()
    }

    pub fn map_to_presentation(model: GeneratedPaymentRequest) -> Self {
        GeneratedPaymentRequestItem::new(model.get_label().to_string(), *model.get_amount(), model.get_address().to_string())
    }

}

#[derive(Debug, PartialEq)]
pub struct PaymentDetailsItem {
    address: String,
    label: String,
    amount: f64,
    status: Status
}

impl<'a> PaymentDetailsItem {
    pub fn new(label: String, amount: f64, address: String, status_id: i32) -> Self {
        let _status = match status_id {
            1 => Status::Success,
            2 => Status::Failed,
            3 => Status::Pending,
            _ => Status::Suspended
        };
        Self {
            label: label,
            amount: amount,
            address: address,
            status: _status
        }
    }
    pub fn get_label(&self) -> &str {
        self.label.as_str()
    }
    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }
    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn get_status_id(&self) -> &Status {
        &self.status
    }
    pub fn map_to_presentation(model: PaymentDetails) -> Self {
        PaymentDetailsItem::new(model.get_label().to_string(), *model.get_amount(), model.get_address().to_string(), *model.get_status_id())
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failed,
    Pending,
    Suspended
}

#[derive(Debug, PartialEq)]
pub struct TransactionItem {
    amount: f64,
    transaction_id: String,
    origin_address: String,
    time_received: NaiveDateTime,
    transaction_status: TransactionStatus,
    transaction_type: TransactionType

}

impl TransactionItem {
    pub fn new(amount: f64, transaction_id: String, origin_address: String, time_received: NaiveDateTime, transaction_status_id: i32, transaction_type_id: i32) -> Self {

        let _transaction_status = match transaction_status_id {
            1 => TransactionStatus::Success,
            _ => TransactionStatus::Failed
        };

        let _transaction_type = match transaction_type_id {
            1 => TransactionType::Payment,
            _ => TransactionType::Refund
        };

        Self {
            amount: amount,
            transaction_id: transaction_id,
            origin_address: origin_address,
            time_received: time_received,
            transaction_status: _transaction_status,
            transaction_type: _transaction_type
        }
    }
    pub fn map_to_presentation(model: &Transaction) -> Self {
        TransactionItem::new(*model.get_amount(), model.get_transaction_id().to_string(), model.get_origin_address().to_string(), *model.get_time_received(), *model.get_transaction_status(), *model.get_transaction_type())
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
    pub fn get_origin_address(&self) -> &str {
        self.origin_address.as_str()
    }
    pub fn get_transaction_status(&self) -> &TransactionStatus {
        &self.transaction_status
    }
    pub fn get_transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }
}

#[derive(Debug, PartialEq)]
pub enum TransactionStatus {
    Success,
    Failed
}

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Payment,
    Refund
}


// ---------------------------------------- TEST

#[cfg(test)]
mod tests {
    use crate::presentation::item::*;
    use chrono::naive::*;

    #[test]
    fn translating_status_to_enum_status() {
        let payment_details_model = PaymentDetails::new("Test".to_string(), 99.99, "Address".to_string(), 1);
        let payment_details_item = PaymentDetailsItem::map_to_presentation(payment_details_model);

        assert_eq!(payment_details_item.status, Status::Success);
    }

    #[test]
    fn translating_transaction_status_to_enum_transaction_status() {
        let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        let transaction_model = Transaction::new(10.0, "Test".to_string(), "Address".to_string(), 1, dt, 1);
        let transaction_item = TransactionItem::map_to_presentation(&transaction_model);

        assert_eq!(transaction_item.transaction_status, TransactionStatus::Success);
    }

    #[test]
    fn translating_transaction_type_to_enum_transaction_type() {
        let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        let transaction_model = Transaction::new(10.0, "Test".to_string(), "Address".to_string(), 1, dt, 1);
        let transaction_item = TransactionItem::map_to_presentation(&transaction_model);

        assert_eq!(transaction_item.transaction_type, TransactionType::Payment);
    }
}
