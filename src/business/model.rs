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

pub struct PaymentDetails<'a> {
    address: &'a str,
    required_amount: &'a f32,
    total_amount_payed: &'a f32,
    date: &'a chrono::NaiveDate,
    is_successful: &'a bool
}
