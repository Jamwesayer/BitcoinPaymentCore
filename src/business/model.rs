// PaymentRequest
#[derive(Debug, PartialEq)]
pub struct PaymentRequest {
    label: String,
    amount: f64,
    store_id: i32
}

impl PaymentRequest {
    pub fn new(label: String, amount: f64, store_id: i32) -> Self {
        PaymentRequest {
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

}

#[derive(Debug, PartialEq, Clone)]
pub struct PaymentWindowSearch {
    label: String,
    store_id: i32
}

impl PaymentWindowSearch {
    pub fn new(label: String, store_id: i32) -> Self {
        PaymentWindowSearch {
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

}

// GeneratedPaymentRequest
#[derive(Debug, PartialEq)]
pub struct GeneratedPaymentRequest {
    label: String,
    amount: f64,
    address: String
}

impl GeneratedPaymentRequest {
    pub fn new(label: String, amount: f64, address: String) -> Self {
        GeneratedPaymentRequest {
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
        self.address.as_str()
    }
}

#[derive(Debug, PartialEq)]
pub struct PaymentDetails {
    address: String,
    label: String,
    amount: f64,
    status_id: i32
}

impl PaymentDetails {
    pub fn new(label: String, amount: f64, address: String, status_id: i32) -> Self {
        Self {
            label: label,
            amount: amount,
            address: address,
            status_id: status_id
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
    pub fn get_status_id(&self) -> &i32 {
        &self.status_id
    }
}

use chrono::NaiveDateTime;

// TransactionEntity
#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    amount: f64,
    transaction_id: String,
    origin_address: String,
    transaction_type: i32,
    time_received: NaiveDateTime,
    transaction_status_id: i32
}

impl Transaction {
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
}

pub struct Store {
    name: String,
    address: String,
    wallet_address: String
}

impl Store {
    pub fn new(name: String, address: String, wallet_address: String) -> Self {
        Self {
            name: name,
            address: address,
            wallet_address: wallet_address
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_address(&self) -> &str {
        self.address.as_str()
    }

    pub fn get_wallet_address(&self) -> &str {
        self.wallet_address.as_str()
    }
}
