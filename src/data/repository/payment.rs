use crate::data::idatasource::{{PaymentNetwork, PaymentDatabase, IPaymentNetworkDataSource, IPaymentDatabaseDataSource}};
use crate::data::entity::payment::{PaymentRequestEntity};
use crate::business::irepository::{IPaymentRepository};
use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

pub struct PaymentRepository<'a> {
    pub payment_network_datasource: &'a dyn IPaymentNetworkDataSource,
    pub payment_database_datasource: &'a dyn IPaymentDatabaseDataSource<'a>
}

impl<'a> IPaymentRepository for PaymentRepository<'a> {
    fn create_payment_window(&self, payment_request: PaymentRequest) -> Result<GeneratedPaymentRequest, String> {
        match self.payment_database_datasource.insert_payment_window(&PaymentRequestEntity::map_to_entity(payment_request)) {
            Ok(generated_payment_request_entity) => Ok(generated_payment_request_entity.map_to_business()),
            Err(e) => Err(e)
        }

    }

    fn check_payment_status(&self, label: &str) -> Result<PaymentDetails, String> {
        match self.payment_database_datasource.check_payment_window_status(label) {
            Ok(payment_details_entity) => Ok(payment_details_entity.map_to_business()),
            Err(e) => Err(e)
        }
    }

    fn refund(&self, label: &str) -> Result<String, String> {
        self.payment_network_datasource.send_refund(label)
    }

}

// Set defaults
// impl Default for PaymentRepository {
//     fn default() -> Self {
//         Self {
//             payment_network_datasource: PaymentNetwork::new(),
//             payment_database_datasource: PaymentDatabase::new()
//         }
//     }
// }
