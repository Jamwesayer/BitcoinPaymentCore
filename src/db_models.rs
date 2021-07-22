use chrono;
use crate::schema::{payment_window, transaction, shop};

#[derive(Insertable)]
#[table_name = "payment_window"]
pub struct NewPaymentRequest<'a> {
    pub label: &'a str,
    pub amount: &'a f64,
    pub store_id: &'a i32
}

#[derive(Identifiable, Queryable)]
#[table_name = "payment_window"]
pub struct PaymentWindow {
    pub id: i32,
    pub label: String,
    pub amount: f64,
    pub date: chrono::NaiveDate,
    pub status_id: i32,
    pub store_id: i32
}

#[derive(Queryable)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub wallet_address: String
}

#[derive(Insertable)]
#[table_name = "shop"]
pub struct NewStore<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub wallet_address: &'a str
}

#[derive(Insertable)]
#[table_name = "transaction"]
pub struct NewTransaction<'a> {
    pub amount: &'a f64,
    pub hash: &'a str,
    pub from_address: &'a str,
    pub date: &'a chrono::NaiveDateTime,
    pub transaction_type_id: &'a i32,
    pub payment_window_id: &'a i32
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(PaymentWindow. foreign_key="payment_window_id")]
#[table_name = "transaction"]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub hash: String,
    pub from_address: String,
    pub date: chrono::NaiveDateTime,
    pub transaction_type_id: i32,
    pub transaction_status_id: i32,
    pub payment_window_id: i32
}
