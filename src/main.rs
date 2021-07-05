use payment_core::presentation::controller_service::PaymentControllerService;
use payment_core::presentation::item::*;
use payment_core::data::database_util;

#[tokio::main]
async fn main() {


    database_util::get_amount_of_transactions_for_shop(&1);
}
