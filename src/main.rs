use tokio::time::sleep;

use payment_core::data::idatasource::ITransactionNetworkDataSource;
use payment_core::business::usecase::payment::PaymentUseCase;
use payment_core::business::usecase::transaction::TransactionUseCase;
use payment_core::data::idatasource::PaymentDatabase;
use payment_core::data::idatasource::PaymentNetwork;
use payment_core::data::idatasource::TransactionNetwork;
use payment_core::presentation::item::PaymentRequestItem;
use payment_core::data::repository::payment::PaymentRepository;
use payment_core::data::repository::transaction::TransactionRepository;
use payment_core::data::blockchain_util::import_address;

#[tokio::main]
async fn main() {
    // let test = PaymentRequestItem::new("joost".to_string(), 0.001, 1);
    let repo: PaymentRepository = Default::default();
    // // let repo: PaymentRepository = PaymentRepository {
    // //     payment_database_datasource: &PaymentDatabase::new(),
    // //     payment_network_datasource: &PaymentNetwork::new()
    // // };
    // let usecase = PaymentUseCase::new(Box::new(repo));
    // // println!("{:?}", usecase.create_payment_window(test));
    // println!("{:?}", usecase.check_payment_status("joost"));
    // match import_address("bcrt1q2mtlj023wx0k38wv5vjk4a6msukqgrxkgp6hmt") {
    //     Ok(_) => println!("Created"),
    //     Err(e) => println!("{}", e)
    // }

    let usecase = TransactionUseCase::default();
    usecase.follow_transaction("testtesttest".to_string(), 10.0).await;
    // let usecase = TransactionUseCase::newmain())

    // transaction.follow_transactions_for_label("test12345", 2000.0);
    // thread::sleep(Duration::from_millis(5000));
    // transaction.follow_transactions_for_label("test", 1000.0);
    // transaction.follow_transactions_for_label("test12345", 2000.0);
}
