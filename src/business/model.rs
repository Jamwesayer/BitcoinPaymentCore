// PaymentRequest
pub struct PaymentRequest {
    label: Option<String>,
    amount: f32,
    store_id: i32
}

impl<'a> PaymentRequest {
    pub fn new(label: Option<String>, amount: f32, store_id: i32) -> Self {
        PaymentRequest {
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

}

// GeneratedPaymentRequest
#[derive(Debug)]
pub struct GeneratedPaymentRequest {
    label: Option<String>,
    amount: f32,
    address: String
}

impl<'a> GeneratedPaymentRequest {
    pub fn new(label: Option<String>, amount: f32, address: String) -> Self {
        GeneratedPaymentRequest {
            label: label,
            amount: amount,
            address: address
        }
    }
    pub fn get_label(&self) -> Option<&str> {
        Some(self.label.as_ref().unwrap().as_str())
    }
    pub fn get_amount(&self) -> &f32 {
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
    amount: f32,
    status_id: i32
}

impl<'a> PaymentDetails {
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
}
