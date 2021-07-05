use crate::data::idatasource::{IPaymentNetworkDataSource, IPaymentDatabaseDataSource, PaymentNetwork, PaymentDatabase};
use crate::data::entity::payment::{PaymentRequestEntity};
use crate::business::irepository::{IPaymentRepository};
use crate::business::model::*;

pub struct PaymentRepository {
    payment_network_datasource: Box<dyn IPaymentNetworkDataSource>,
    payment_database_datasource: Box<dyn IPaymentDatabaseDataSource>
}

impl PaymentRepository {
    pub fn new(payment_network_datasource: Box<dyn IPaymentNetworkDataSource>, payment_database_datasource: Box<dyn IPaymentDatabaseDataSource>) -> Self {
        Self {
            payment_network_datasource: payment_network_datasource,
            payment_database_datasource: payment_database_datasource
        }
    }
}

impl IPaymentRepository for PaymentRepository {
    fn create_payment_window(&self, payment_request: PaymentRequest) -> Result<GeneratedPaymentRequest, String> {
        match self.payment_database_datasource.insert_payment_window(&PaymentRequestEntity::map_to_entity(payment_request)) {
            Ok(generated_payment_request_entity) => {
                Ok(generated_payment_request_entity.map_to_business())
            },
            Err(e) => Err(e)
        }

    }

    fn check_payment_status(&self, label: &str) -> Result<PaymentDetails, String> {
        match self.payment_database_datasource.check_payment_window_status(label) {
            Ok(payment_details_entity) => Ok(payment_details_entity.map_to_business()),
            Err(e) => Err(e)
        }
    }

    fn refund(&self, label: &str) -> Result<Vec<Transaction>, String> {
        match self.payment_database_datasource.get_payment_window_by_label(label) {
            Ok(_) => {
                match self.payment_network_datasource.send_refund(label) {
                    Ok(transaction_entities) => {
                        match self.payment_database_datasource.suspend_payment_window(label) {
                            Ok(_) => {
                                let mut transactions: Vec<Transaction> = Vec::new();
                                for transaction_entity in transaction_entities {
                                    transactions.push(transaction_entity.map_to_business())
                                };
                                Ok(transactions)
                            },
                            Err(e) => Err(e)
                        }

                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }

    fn suspend_payment_window(&self, label: &str) -> Result<(), String> {
        self.payment_database_datasource.suspend_payment_window(label)
    }
}

// Set defaults
impl Default for PaymentRepository {
    fn default() -> Self {
        Self {
            payment_network_datasource: Box::new(PaymentNetwork::default()),
            payment_database_datasource: Box::new(PaymentDatabase::default())
        }
    }
}
