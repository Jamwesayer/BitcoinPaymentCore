use crate::data::idatasource::{IPaymentNetworkDataSource, IPaymentDatabaseDataSource, PaymentNetwork, PaymentDatabase};
use crate::data::entity::payment::*;
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
        let generated_payment_request_entity = self.payment_database_datasource.insert_payment_window(&PaymentRequestEntity::map_to_entity(&payment_request))?;
        let address = self.payment_network_datasource.create_payment_window(&payment_request.get_label())?;

        Ok(generated_payment_request_entity.map_to_business(address))
    }

    fn check_payment_status(&self, payment_search_model: PaymentWindowSearch) -> Result<PaymentDetails, String> {
        let payment_details_entity = self.payment_database_datasource.check_payment_window_status(PaymentWindowSearchEntity::map_to_entity(&payment_search_model))?;
        Ok(payment_details_entity.map_to_business())
    }

    fn refund(&self, payment_search_model: PaymentWindowSearch) -> Result<Vec<Transaction>, String> {
        self.payment_database_datasource.get_payment_window_by_label(PaymentWindowSearchEntity::map_to_entity(&payment_search_model))?;
        let transaction_entities = self.payment_network_datasource.send_refund(&payment_search_model.get_label())?;

        let mut transactions: Vec<Transaction> = Vec::new();
        for transaction_entity in transaction_entities {
            transactions.push(transaction_entity.map_to_business())
        };
        Ok(transactions)
    }

    fn suspend_payment_window(&self, payment_search_model: PaymentWindowSearch) -> Result<(), String> {
        self.payment_database_datasource.suspend_payment_window(PaymentWindowSearchEntity::map_to_entity(&payment_search_model))
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
