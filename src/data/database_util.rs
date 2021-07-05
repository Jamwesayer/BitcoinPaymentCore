use crate::db_models::*;
use dotenv::dotenv;
use std::env;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error::*;
use diesel::result::Error;

use super::entity::payment::*;
use super::entity::transaction::*;

/// Creates a connection to the database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_store_wallet_by_id(_id: &i32) -> Result<Shop, Error> {
    use crate::schema::shop::dsl::{shop, id};
    shop
    .filter(id.eq(_id))
    .limit(1)
    .get_result::<Shop>(&establish_connection())
}

/// Inserts a paymentrequest object in the database
///
/// # Arguments
///
/// * `enttiy` - A GeneratedPaymentRequestEntity object
pub fn insert_payment_window(entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, Error> {
    use crate::schema::payment_window::dsl::{payment_window, id};
    let new_payment_window = NewPaymentRequest {label: entity.get_label(), amount: entity.get_amount(), store_id: entity.get_store_id()};

    diesel::insert_into(payment_window)
        .values(&new_payment_window)
        .execute(&establish_connection())
        .expect("Error inserting payment window");

    let payment_window_row: PaymentWindow = payment_window.order(id.desc()).first(&establish_connection()).unwrap();

    match get_store_wallet_by_id(&payment_window_row.store_id) {
        Ok(store_row) => Ok(GeneratedPaymentRequestEntity::new(payment_window_row.label, payment_window_row.amount, store_row.wallet_address, payment_window_row.store_id)),
        Err(e) => Err(e)
    }
}

/// Checks the status for a certain address using a search query with a label
///
/// # Arguments
///
/// * `label` - An &str value
pub fn check_payment_window_status(_label: &str) -> Result<PaymentWindow, Error> {
    use crate::schema::payment_window::dsl::{payment_window, label};

    payment_window
        .filter(label.eq(_label))
        .limit(1)
        .get_result::<PaymentWindow>(&establish_connection())

}

pub fn get_payment_window_by_label(_label: &str) -> Result<(), Error> {
    use crate::schema::payment_window::dsl::{payment_window, payment_status_id};

    match check_payment_window_status(_label) {
        Ok(row) => {
            if row.status_id.eq(&2) || row.status_id.eq(&4) {
                Err(NotFound)
            } else {
                 match diesel::update(payment_window.find(row.id))
                .set(payment_status_id.eq(4))
                .execute(&establish_connection()) {
                    Ok(item) => Ok(()),
                    Err(e) => Err(e)
                }
            }
        },
        Err(e) => Err(e)
    }

}
