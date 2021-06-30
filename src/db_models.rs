// #[derive(Queryable)]
// pub struct PaymentWindow {
//     pub id: i32,
//     pub name: String
// }

use chrono;
use crate::schema::{payment_window};

#[derive(Insertable)]
#[table_name = "payment_window"]
pub struct NewPaymentRequest<'a> {
    pub label: &'a str,
    pub amount: &'a f32,
    pub store_id: &'a i32
}

#[derive(Queryable)]
pub struct PaymentWindow {
    pub id: i32,
    pub label: String,
    pub amount: f32,
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
