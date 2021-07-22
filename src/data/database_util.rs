use crate::db_models::*;
use dotenv::dotenv;
use std::env;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use super::entity::payment::*;
use super::entity::transaction::*;
use super::entity::error::DataSourceError;
use super::entity::store::*;

/// Creates a connection to the database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn register_store(store_register_entity: StoreEntity) -> Result<StoreEntity, DataSourceError> {
    use crate::schema::shop::dsl::{shop};
    let new_store = NewStore {name: store_register_entity.get_name(), address: store_register_entity.get_address(), wallet_address: store_register_entity.get_wallet_address()};
    
    match diesel::insert_into(shop)
        .values(&new_store)
        .execute(&establish_connection()) {
            Ok(_) => Ok(store_register_entity),
            Err(e) => Err(DataSourceError::KnownError(e))
        }
}

pub fn get_store_wallet_by_id(_id: &i32) -> Result<Shop, DataSourceError> {
    use crate::schema::shop::dsl::{shop, id};
    match shop
    .filter(id.eq(_id))
    .limit(1)
    .get_result::<Shop>(&establish_connection()) {
        Ok(shop_db) => Ok(shop_db),
        Err(e) => return Err(DataSourceError::KnownError(e))
    }
}

/// Inserts a paymentrequest object in the database
///
/// # Arguments
///
/// * `enttiy` - A GeneratedPaymentRequestEntity object
pub fn insert_payment_window(entity: &PaymentRequestEntity) -> Result<GeneratedPaymentRequestEntity, DataSourceError> {
    use crate::schema::payment_window::dsl::{payment_window, id};
    let new_payment_window = NewPaymentRequest {label: entity.get_label(), amount: entity.get_amount(), store_id: entity.get_store_id()};

    match get_payment_window_by_label(entity.get_label(), entity.get_store_id()) {
        Ok(_) => return Err(DataSourceError::AlreadyExistInDatabaseError(entity.get_label().to_string())),
        Err(_) => {}
    }

    let store_row = get_store_wallet_by_id(entity.get_store_id())?;

    match diesel::insert_into(payment_window)
        .values(&new_payment_window)
        .execute(&establish_connection()) {
            Ok(_) => {},
            Err(e) => return Err(DataSourceError::KnownError(e))
        };

    let payment_window_row: PaymentWindow = payment_window.order(id.desc()).first(&establish_connection()).unwrap();

    Ok(GeneratedPaymentRequestEntity::new(payment_window_row.label, payment_window_row.amount, store_row.wallet_address, payment_window_row.store_id))
}

pub fn set_payment_window_to_payed(_label: &str, store_id: &i32) -> Result<(), DataSourceError> {
    use crate::schema::payment_window::dsl::{payment_window, payment_status_id};

    let row = get_payment_window_by_label(_label, store_id)?;

    match diesel::update(payment_window.find(row.id))
    .filter(payment_status_id.eq(3))
    .set(payment_status_id.eq(1))
    .execute(&establish_connection()) {
        Ok(_) => Ok(()),
        Err(e) => return Err(DataSourceError::KnownError(e))
    }
}

/// Supsends a open payment window with the given label
///
/// # Arguments
///
/// * `label` - An &str value
pub fn suspend_payment_window(_label: &str, store_id: &i32) -> Result<(), DataSourceError> {
    use crate::schema::payment_window::dsl::{payment_window, payment_status_id};
    let row = get_payment_window_by_label(_label, store_id)?;

    match diesel::update(payment_window.find(row.id))
    .filter(payment_status_id.eq(3))
    .set(payment_status_id.eq(4))
    .execute(&establish_connection()) {
        Ok(_) => Ok(()),
        Err(e) => return Err(DataSourceError::KnownError(e))
    }

}

/// Get the payment window by label for a specific store
///
/// # Arguments
///
/// * `label` - An &str value
/// * `label` - An &i32 value representing the specific store
pub fn get_payment_window_by_label(_label: &str, _store_id: &i32) -> Result<PaymentWindow, DataSourceError> {
    use crate::schema::payment_window::dsl::{payment_window, label, store_id};

    match payment_window
        .filter(label.eq(_label))
        .filter(store_id.eq(_store_id))
        .limit(1)
        .get_result::<PaymentWindow>(&establish_connection()) {
            Ok(payment_window_db) => Ok(payment_window_db),
            Err(e) => Err(DataSourceError::KnownError(e))
        }
}

/// Adds transactions tot he database
///
/// # Arguments
///
/// * `_label` - An &str value to find the payment window that will be used to associate the id
/// * `_transactions` - A list of transactions entities which will be added tot he database
pub fn insert_transactions(_label: &str, store_id: &i32,  _transactions: Vec::<TransactionEntity>) -> Result<(), DataSourceError> {
    use crate::schema::transaction::dsl::{transaction};

    let mut transaction_db_models: Vec::<NewTransaction> = Vec::new();

    let payment_window = get_payment_window_by_label(_label, store_id)?;
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
}

pub fn get_all_transactions(_store_id: &i32) -> Result<Vec<Transaction>, Error> {
    use crate::schema::transaction::dsl::{transaction, hash, id, amount, from_address, date, transaction_type_id, transaction_status_id, payment_window_id};
    use crate::schema::payment_window::dsl::{payment_window, store_id};

    let transactions: Vec<Transaction> = transaction.inner_join(payment_window).select((id, amount, hash, from_address, date, transaction_type_id, transaction_status_id, payment_window_id)).filter(store_id.eq(_store_id)).load(&establish_connection()).expect("Couldn't get all transactions for store");
    Ok(transactions)
}

pub fn get_transaction_by_transaction_id(_transaction_id: &str) -> Result<Transaction, Error> {
    use crate::schema::transaction::dsl::{transaction, hash};

    transaction
        .filter(hash.eq(_transaction_id))
        .limit(1)
        .get_result::<Transaction>(&establish_connection())
}

pub fn get_amount_of_transactions_for_shop(shop_id: &i32) -> Result<i64, Error> {
    use crate::schema::transaction::dsl::{transaction, id as other_id};
    use crate::schema::payment_window::dsl::{payment_window, store_id};

    use crate::diesel::dsl::count;

    let count: i64 = transaction.inner_join(payment_window).select(count(other_id)).filter(store_id.eq(shop_id)).first(&establish_connection()).expect(format!("Error getting the total of transaction {}", shop_id).as_str());
    Ok(count)
}
