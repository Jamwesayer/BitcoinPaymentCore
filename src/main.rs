use payment_core::presentation::controller::*;
use payment_core::presentation::item::*;

use std::thread;
use std::time::Duration;

use clap::{Arg, App, SubCommand};

#[tokio::main]
async fn main() {

    let payment_controller = PaymentController::default();

    let matches = App::new("BTC Payment System")
        .version("0.1")
        .author("James Bal")
        .about("Handle BTC wallets, transaction and overviews")
        .subcommand(
            SubCommand::with_name("create-order")
            .about("Create an order")
            .version("0.1")
            .author("James Bal")
            .arg(Arg::with_name("label")
                .required(true)
                .index(1))
            .arg(Arg::with_name("amount")
                .required(true)
                .index(2))
        )
        .subcommand(
            SubCommand::with_name("check-order")
            .about("Check a specific order")
            .version("0.1")
            .author("James Bal")
            .arg(Arg::with_name("label")
                .required(true)
                .index(1))
        )
        .subcommand(
            SubCommand::with_name("cancel-order")
            .about("Refund an order, this means that the order is manually cancelled")
            .version("0.1")
            .author("James Bal")
            .arg(Arg::with_name("label")
                .required(true)
                .index(1))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create-order") {
        let label = matches.value_of("label").unwrap().to_string();
        let amount = matches.value_of("amount").unwrap().to_string().parse::<f64>().unwrap();
        let store_id = get_current_store_id();

        let payment_request_item = PaymentRequestItem::new(label, amount, store_id);
        payment_controller.create_payment_window(payment_request_item).await;
        thread::sleep(Duration::from_millis(1000));
    }

    if let Some(matches) = matches.subcommand_matches("check-order") {
        let label = matches.value_of("label").unwrap().to_string();
        let store_id = get_current_store_id();

        let search_item = PaymentWindowSearchItem::new(label, store_id);
        payment_controller.check_payment_status(search_item);
    }

    if let Some(matches) = matches.subcommand_matches("cancel-order") {
        let label = matches.value_of("label").unwrap().to_string();
        let store_id = get_current_store_id();

        let search_item = PaymentWindowSearchItem::new(label, store_id);
        payment_controller.refund(search_item);
    }
}

fn get_current_store_id() -> i32 {
    1
}
