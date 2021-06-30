use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

#[derive(Debug, PartialEq)]
pub struct PaymentRequestItem {
    label: String,
    amount: f32,
    store_id: i32
}

impl<'a> PaymentRequestItem {
    pub fn new(label: String, amount: f32, store_id: i32) -> Self {
        PaymentRequestItem {
            label: label,
            amount: amount,
            store_id: store_id
        }
    }
    pub fn get_label(&self) -> &str {
        self.label.as_str()
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

#[derive(Debug, PartialEq)]
pub struct GeneratedPaymentRequestItem {
    label: String,
    amount: f32,
    address: String
}

impl<'a> GeneratedPaymentRequestItem {
    pub fn new(label: String, amount: f32, address: String) -> Self {
        GeneratedPaymentRequestItem {
            label: label,
            amount: amount,
            address: address
        }
    }
    pub fn get_label(&self) -> &str {
        self.label.as_str()
    }
    pub fn get_amount(&self) -> &f32 {
        &self.amount
    }
    pub fn get_store_id(&self) -> &str {
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
    amount: f32,
    status: Status
}

impl<'a> PaymentDetailsItem {
    pub fn new(label: String, amount: f32, address: String, status_id: i32) -> Self {
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
    pub fn get_amount(&self) -> &f32 {
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

