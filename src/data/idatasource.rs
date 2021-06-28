extern crate bitcoincore_rpc;
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

use crate::data::entity::payment::{PaymentRequestEntity, GeneratedPaymentRequestEntity};

// Payment
pub trait IPaymentDatabaseDataSource<'a> {

}

pub struct PaymentDatabase {
}

impl<'a> PaymentDatabase {
    pub fn new() -> Self {
        Self { /* fields */ }
    }
}

impl<'a> IPaymentDatabaseDataSource<'a> for PaymentDatabase {
}

pub trait IPaymentNetworkDataSource<'a> {
    fn create_payment_window(&self, payment_window_entity: PaymentRequestEntity) -> GeneratedPaymentRequestEntity;
    // fn sendRefund(&self, origin: Address);
}

pub struct PaymentNetwork {}

impl PaymentNetwork {
    pub fn new() -> Self {
        Self { /* fields */ }
    }
}

impl<'a> IPaymentNetworkDataSource<'a> for PaymentNetwork {

    fn create_payment_window(&self, payment_window_entity: PaymentRequestEntity) -> GeneratedPaymentRequestEntity {
        //let address = open_connection().get_new_address(payment_window_entity.get_label(), None).unwrap();
        let address: String="tet".to_string();
        GeneratedPaymentRequestEntity::new(Some(payment_window_entity.get_label().unwrap().to_string()), *payment_window_entity.get_amount(), address)
    }
}
//
// pub struct PaymentDatabase {}
//
// impl IPaymentDataSource for PaymentDatabase {
//
// fn create_payment_window(&self, _: data::entity::payment::PaymentRequestEntity<'_>) { todo!() }
// }

fn open_connection() -> bitcoincore_rpc::Client {
    return Client::new("http://localhost:18443".to_string(),
                          Auth::UserPass("test".to_string(),
                                         "test".to_string())).unwrap();
}
