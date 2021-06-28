use crate::business::model::{PaymentRequest, GeneratedPaymentRequest};

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

    pub fn get_label(&self) -> Option<&str> {
        Some(self.label.as_ref().unwrap().as_str())
    }
    pub fn get_amount(&self) -> &f32 {
        &self.amount
    }
    pub fn get_address(&self) -> &str {
        &self.address
    }
}
