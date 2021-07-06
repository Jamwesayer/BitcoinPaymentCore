use crate::business::model::*;

// PaymentRequestEntity
#[derive(Debug, PartialEq)]
pub struct PaymentRequestEntity {
    label: String,
    amount: f64,
    store_id: i32
}

impl PaymentRequestEntity {
    pub fn map_to_entity(model: PaymentRequest) -> Self {
        PaymentRequestEntity {
            label: model.get_label().to_string(),
            amount: *model.get_amount(),
            store_id: *model.get_store_id()
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
pub struct PaymentWindowSearchEntity {
    label: String,
    store_id: i32
}

impl PaymentWindowSearchEntity {
    pub fn new(label: String, store_id: i32) -> Self {
        PaymentWindowSearchEntity {
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
    pub fn map_to_entity(model: PaymentWindowSearch) -> Self {
        PaymentWindowSearchEntity {
            label: model.get_label().to_string(),
            store_id: *model.get_store_id()
        }
    }
}

// GeneratedPaymentRequestEntity
#[derive(Debug, PartialEq)]
pub struct GeneratedPaymentRequestEntity {
    label: String,
    amount: f64,
    address: String,
    store_id: i32
}

impl GeneratedPaymentRequestEntity {
    pub fn new(label: String, amount: f64, address: String, store_id: i32) -> Self {
        Self {
            label: label,
            amount: amount,
            address: address,
            store_id: store_id
        }
    }

    pub fn map_to_business(&self, address: String) -> GeneratedPaymentRequest {
        GeneratedPaymentRequest::new(self.get_label().to_string(), *self.get_amount(), address)
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
    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }
    pub fn get_store_id(&self) -> &i32 {
        &self.store_id
    }
}

// PaymentDetailsEntity
#[derive(Debug, PartialEq)]
pub struct PaymentDetailsEntity {
    address: String,
    label: String,
    amount: f64,
    status_id: i32
}

impl PaymentDetailsEntity {
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

    pub fn map_to_business(&self) -> PaymentDetails {
        PaymentDetails::new(self.get_label().to_string(), *self.get_amount(), self.get_address().to_string(), *self.get_status_id())
    }
}
