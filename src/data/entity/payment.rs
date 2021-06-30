use crate::business::model::{PaymentRequest, GeneratedPaymentRequest};
use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

// PaymentRequestEntity
pub struct PaymentRequestEntity {
    label: Option<String>,
    amount: f32,
    store_id: i32
}

impl<'a> PaymentRequestEntity {
    pub fn map_to_entity(model: PaymentRequest) -> Self {
        PaymentRequestEntity {
            label: Some(model.get_label().unwrap().to_string()),
            amount: *model.get_amount(),
            store_id: *model.get_store_id()
        }
    }
    pub fn get_label(&self) -> Option<&str> {
        Some(self.label.as_ref().unwrap().as_str())
    }
    pub fn get_amount(&self) -> &f32 {
        &self.amount
    }
    pub fn get_store_id(&self) -> &i32 {
        &self.store_id
    }
}

// GeneratedPaymentRequestEntity
pub struct GeneratedPaymentRequestEntity {
    label: Option<String>,
    amount: f32,
    address: String
}

impl<'a> GeneratedPaymentRequestEntity {
    pub fn new(label: Option<String>, amount: f32, address: String) -> Self {
        Self {
            label: label,
            amount: amount,
            address: address
        }
    }

    pub fn map_to_business(&self) -> GeneratedPaymentRequest {
        GeneratedPaymentRequest::new(Some(self.get_label().unwrap().to_string()), *self.get_amount(), self.get_address().to_string())
    }
// PaymentDetailsEntity
#[derive(Debug, PartialEq)]
pub struct PaymentDetailsEntity {
    address: String,
    label: String,
    amount: f32,
    status_id: i32
}

impl<'a> PaymentDetailsEntity {
    pub fn new(label: String, amount: f32, address: String, status_id: i32) -> Self {
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
    pub fn get_amount(&self) -> &f32 {
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
