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

/// Supsends a open payment window with the given label
///
/// # Arguments
///
/// * `label` - An &str value
pub fn suspend_payment_window(_label: &str) -> Result<(), Error> {
    use crate::schema::payment_window::dsl::{payment_window, payment_status_id};

    match get_payment_window_by_label(_label) {
        Ok(row) => {
            if row.status_id.eq(&2) || row.status_id.eq(&4) {
                Err(NotFound)
            } else {
                 match diesel::update(payment_window.find(row.id))
                .set(payment_status_id.eq(4))
                .execute(&establish_connection()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            }
        },
        Err(e) => Err(e)
    }
}

/// Get the payment window by label
///
/// # Arguments
///
/// * `label` - An &str value
pub fn get_payment_window_by_label(_label: &str) -> Result<PaymentWindow, Error> {
    use crate::schema::payment_window::dsl::{payment_window, label};

    payment_window
        .filter(label.eq(_label))
        .limit(1)
        .get_result::<PaymentWindow>(&establish_connection())
}

/// Adds transactions tot he database
///
/// # Arguments
///
/// * `_label` - An &str value to find the payment window that will be used to associate the id
/// * `_transactions` - A list of transactions entities which will be added tot he database
pub fn insert_transactions(_label: &str, _transactions: Vec::<TransactionEntity>) -> Result<(), Error> {
    use crate::schema::transaction::dsl::{transaction};

    let mut transaction_db_models: Vec::<NewTransaction> = Vec::new();

    match get_payment_window_by_label(_label) {
        Ok(payment_window) => {
            let payment_window_id = &payment_window.id;
            for _transaction in &_transactions {
                transaction_db_models.push(NewTransaction {
                    amount: _transaction.get_amount(),
                    hash: _transaction.get_transaction_id(),
                    from_address: _transaction.get_origin_address(),
                    date: _transaction.get_time_received(),
                    transaction_type_id: _transaction.get_transaction_type(),
                    payment_window_id: payment_window_id
                })
            }

            diesel::insert_into(transaction)
                .values(&transaction_db_models)
                .execute(&establish_connection())
                .expect("Error inserting transactions");

            Ok(())
        },
        Err(e) => Err(e)
    }
}

pub fn get_transaction_by_transaction_id(_transaction_id: &str) -> Result<Transaction, Error> {
    use crate::schema::transaction::dsl::{transaction, hash};

    transaction
        .filter(hash.eq(_transaction_id))
        .limit(1)
        .get_result::<Transaction>(&establish_connection())
}

pub fn get_amount_of_transactions_for_shop(shop_id: &i32) -> Result<i64, Error> {
    use crate::schema::transaction::dsl::{transaction, payment_window_id, id as other_id};
    use crate::schema::payment_window::dsl::{payment_window, store_id, id, label};

    use crate::db_models::*;
    use crate::diesel::dsl::count;

    let count: i64 = transaction.inner_join(payment_window).select(count(other_id)).filter(store_id.eq(shop_id)).first(&establish_connection()).expect(format!("Error getting the total of transaction {}", shop_id).as_str());
    Ok(count)
}
