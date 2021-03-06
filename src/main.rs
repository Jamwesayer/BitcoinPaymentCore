use payment_core::business::model::Store;
use payment_core::presentation::controller::*;
use payment_core::presentation::item::*;

use std::thread;
use std::time::Duration;

use clap::{Arg, App, SubCommand};

const VERSION: &str = "0.1";

#[tokio::main]
async fn main() {

    let payment_controller = PaymentController::default();
    let transaction_controller = TransactionController::default();

    let matches = App::new("BTC Payment System")
        .version(VERSION)
        .author("James Bal")
        .about("Handle BTC wallets, transaction and overviews")
        .subcommand(
            SubCommand::with_name("create-order")
            .about("Create an order")
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
            .arg(Arg::with_name("label")
                .required(true)
                .index(1))
        )
        .subcommand(
            SubCommand::with_name("cancel-order")
            .about("Refund an order, this means that the order is manually cancelled")
            .arg(Arg::with_name("label")
                .required(true)
                .index(1))
        )
        .subcommand(
            SubCommand::with_name("get-transactions")
            .about("Retrieve all transactions")
            .version("0.1")
            .author("James Bal")
        )
        .subcommand(
            SubCommand::with_name("register")
            .about("Retrieve all transactions")
            .version("0.1")
            .author("James Bal")
            .arg(Arg::with_name("name")
                .required(true)
                .index(1))
            // .arg(Arg::with_name("address")
            //     .required(true)
            //     .index(1))
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
        payment_controller.suspend_payment_window(search_item);
    }

    if let Some(matches) = matches.subcommand_matches("get-transactions") {
        transaction_controller.get_all_transactions(&get_current_store_id());
    }

    if let Some(matches) = matches.subcommand_matches("register") {
        let name = matches.value_of("name").unwrap().to_string();
        // let address = matches.value_of("address").unwrap().to_string();
        // let store = Store::new(name, address, "test".to_string());

        // register_controller.register(store)
    }
}

fn get_current_store_id() -> i32 {
    1
}
