use chrono;
use crate::schema::{payment_window, transaction};

#[derive(Insertable)]
#[table_name = "payment_window"]
pub struct NewPaymentRequest<'a> {
    pub label: &'a str,
    pub amount: &'a f64,
    pub store_id: &'a i32
}

#[derive(Queryable)]
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
#[table_name = "transaction"]
pub struct NewTransaction<'a> {
    pub amount: &'a f64,
    pub hash: &'a str,
    pub from_address: &'a str,
    pub date: chrono::NaiveDateTime,
    pub payment_window_id: &'a i32
}
