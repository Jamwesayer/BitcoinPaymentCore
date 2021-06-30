use payment_core::data::repository::payment::PaymentRepository;
use payment_core::data::idatasource::IPaymentNetworkDataSource;
use payment_core::data::entity::payment::PaymentDetailsEntity;
use payment_core::data::entity::payment::PaymentRequestEntity;
use payment_core::data::entity::payment::GeneratedPaymentRequestEntity;
use payment_core::data::idatasource::IPaymentDatabaseDataSource;

pub struct MockPaymentDatabase {

}

impl<'a> IPaymentDatabaseDataSource<'a> for MockPaymentDatabase {

    fn insert_payment_window(&self, request_entity: &PaymentRequestEntity) -> std::result::Result<GeneratedPaymentRequestEntity, std::string::String> {
        if request_entity.get_label().eq("ThisShouldWork") {
            Ok(GeneratedPaymentRequestEntity::new(request_entity.get_label().to_string(), *request_entity.get_amount(), request_entity.get_label().to_string() + "New", *request_entity.get_store_id()))
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
    fn check_payment_window_status(&self, label: &str) -> std::result::Result<PaymentDetailsEntity, std::string::String> {
        if label == "ThisShouldWork" {
            Ok(PaymentDetailsEntity::new(label.to_string(),99.99, label.to_string() + "New", 1))
        } else {
            Err("ThisIsAnError".to_string())
        }
    }
}

pub struct MockPaymentNetwork {

}

impl<'a> IPaymentNetworkDataSource for MockPaymentNetwork {

    fn send_refund(&self, _: &str) -> std::result::Result<std::string::String, std::string::String> { todo!() }
}

pub fn setup_correct_payment_repository() -> PaymentRepository<'static> {
    PaymentRepository {
        payment_database_datasource: &MockPaymentDatabase {},
        payment_network_datasource: &MockPaymentNetwork {}
    }
}
