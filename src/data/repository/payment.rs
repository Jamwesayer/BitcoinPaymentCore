
use crate::data::idatasource::{{PaymentNetwork, PaymentDatabase, IPaymentNetworkDataSource}};
use crate::data::entity::payment::{PaymentRequestEntity};
use crate::business::irepository::{IPaymentRepository};
use crate::business::model::{PaymentRequest, GeneratedPaymentRequest, PaymentDetails};

pub struct PaymentRepository {
    payment_network_datasource: PaymentNetwork,
    payment_database_datasource: PaymentDatabase
}

impl IPaymentRepository for PaymentRepository {
    fn create_payment_window(&self, payment_request: PaymentRequest) -> GeneratedPaymentRequest {
        let generated_payment_request_entity = self.payment_network_datasource.create_payment_window(PaymentRequestEntity::map_to_entity(payment_request));
        generated_payment_request_entity.map_to_business()
    }
    fn check_payment_status(&self, _: std::option::Option<&str>) -> PaymentDetails<'_> { todo!() }
    }

// Set defaults
impl Default for PaymentRepository {
    fn default() -> Self {
        Self {
            payment_network_datasource: PaymentNetwork::new(),
            payment_database_datasource: PaymentDatabase::new()
        }
    }
}
