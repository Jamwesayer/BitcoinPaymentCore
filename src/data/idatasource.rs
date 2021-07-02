use dyn_clone::DynClone;
use crate::data::blockchain_util as blockchain;
use crate::data::database_util as database;
use crate::data::entity::payment::{PaymentRequestEntity, GeneratedPaymentRequestEntity, PaymentDetailsEntity};
use crate::data::entity::transaction::{TransactionEntity};

//------------------------------------------------------------------------------------------------- Payment
pub trait IPaymentDatabaseDataSource {
    fn insert_payment_window(&self, payment_window_entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, String>;
    fn check_payment_window_status(&self, label: &str) -> Result<PaymentDetailsEntity, String>;
    fn get_payment_window_by_label(&self, label: &str) -> Result<(), String>;
}

pub struct PaymentDatabase {}

impl Default for PaymentDatabase {
    fn default() -> Self {
        Self{}
    }
}

impl<'a> IPaymentDatabaseDataSource for PaymentDatabase {
    fn insert_payment_window(&self, payment_window_entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, String> {
        match database::insert_payment_window(payment_window_entity) {
            Ok(generated_payment_request_entity) => Ok(generated_payment_request_entity),
            Err(e) => Err(e.to_string())
        }
    }

    fn check_payment_window_status(&self, label: &str) -> Result<PaymentDetailsEntity, String>{
        if let Ok(payment_window) = database::check_payment_window_status(label) {
            match database::get_store_wallet_by_id(&payment_window.store_id) {
                Ok(store) => {
                    Ok(PaymentDetailsEntity::new(payment_window.label, payment_window.amount, store.wallet_address, payment_window.status_id))
                },
                Err(e) => Err(e.to_string())
            }

        } else {
            Err("Error".to_string())
        }
    }
    fn get_payment_window_by_label(&self, label: &str) -> Result<(), String> {
        match database::get_payment_window_by_label(label) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}

pub trait IPaymentNetworkDataSource {
    fn send_refund(&self, label: &str) -> Result<String, String>;
}

pub struct PaymentNetwork {}

impl Default for PaymentNetwork {
    fn default() -> Self {
        Self{}
    }
}

impl IPaymentNetworkDataSource for PaymentNetwork {

    fn send_refund(&self, label: &str) -> Result<String, String> {
        blockchain::refund(label)
    }
}
            },
            Err(e) => Err(e.to_string())
        }
    }
}