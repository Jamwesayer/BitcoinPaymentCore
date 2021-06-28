mod data;
mod business;
mod presentation;

use crate::payment::PaymentRepository;
use crate::business::usecase::payment::PaymentUseCase;
use crate::presentation::item::PaymentRequestItem;
use crate::data::repository::payment;

fn main() {
    let test = PaymentRequestItem::new(Some("test".to_string()), 12.12, 1);
    let repo: PaymentRepository = Default::default();
    let usecase = PaymentUseCase::new(&repo);
    usecase.createPaymentWindow(test);
}
