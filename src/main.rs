use payment_core::business::usecase::payment::PaymentUseCase;
use payment_core::data::idatasource::PaymentDatabase;
use payment_core::data::idatasource::PaymentNetwork;
use payment_core::presentation::item::PaymentRequestItem;
use payment_core::data::repository::payment::PaymentRepository;

fn main() {
    let test = PaymentRequestItem::new("joost".to_string(), 0.001, 1);
    // let repo: PaymentRepository = Default::default();
    let repo: PaymentRepository = PaymentRepository {
        payment_database_datasource: &PaymentDatabase::new(),
        payment_network_datasource: &PaymentNetwork::new()
    };
    let usecase = PaymentUseCase::new(&repo);
    // println!("{:?}", usecase.create_payment_window(test));
    println!("{:?}", usecase.check_payment_status("joost"));
}
