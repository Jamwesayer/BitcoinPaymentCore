use crate::data::blockchain_util as blockchain;
use crate::data::database_util as database;
use crate::data::entity::payment::{PaymentRequestEntity, GeneratedPaymentRequestEntity, PaymentDetailsEntity};

// Payment
pub trait IPaymentDatabaseDataSource<'a> {
    fn insert_payment_window(&self, payment_window_entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, String>;
    fn check_payment_window_status(&self, label: &str) -> Result<PaymentDetailsEntity, String>;
}

pub struct PaymentDatabase {
}

impl<'a> PaymentDatabase {
    pub fn new() -> Self {
        Self { /* fields */ }
    }
}

impl<'a> IPaymentDatabaseDataSource<'a> for PaymentDatabase {
    fn insert_payment_window(&self, payment_window_entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, String> {
        match database::insert_payment_window(payment_window_entity) {
            Ok(generated_payment_request_entity) => Ok(generated_payment_request_entity),
            Err(e) => Err(e.to_string())
        }
    }

    fn check_payment_window_status(&self, label: &str) -> Result<PaymentDetailsEntity, String>{
        if let Ok(payment_window) = database::check_payment_window_status(label) {
            match database::get_store_wallet_by_id(&payment_window.id) {
                Ok(store) => {
                    Ok(PaymentDetailsEntity::new(payment_window.label, payment_window.amount, store.wallet_address, payment_window.status_id))
                },
                Err(e) => Err(e.to_string())
            }

        } else {
            Err("Error".to_string())
        }
    }
}

pub trait IPaymentNetworkDataSource {
    fn send_refund(&self, label: &str) -> Result<String, String>;
}

pub struct PaymentNetwork {}

impl PaymentNetwork {
    pub fn new() -> Self {
        Self { /* fields */ }
    }
}

impl IPaymentNetworkDataSource for PaymentNetwork {

    fn send_refund(&self, label: &str) -> Result<String, String> {
        match database::get_payment_window_by_label(label) {
            Ok(_) => {
                blockchain::refund(label)
            },
            Err(e) => Err(e.to_string())
        }
    }
}
