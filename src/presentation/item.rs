use crate::business::model::{PaymentRequest};

pub struct PaymentRequestItem {
    label: Option<String>,
    amount: f32,
    store_id: i32
}

impl<'a> PaymentRequestItem {
    pub fn new(label: Option<String>, amount: f32, store_id: i32) -> Self {
        PaymentRequestItem {
            label: label,
            amount: amount,
            store_id: store_id
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

    pub fn map_to_business(self) -> PaymentRequest {
        PaymentRequest::new(self.label, self.amount, self.store_id)
    }

}
