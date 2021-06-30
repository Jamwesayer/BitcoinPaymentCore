use payment_core::presentation::item::GeneratedPaymentRequestItem;
use payment_core::presentation::item::PaymentRequestItem;
use payment_core::presentation::item::PaymentDetailsItem;
use payment_core::business::usecase::payment::PaymentUseCase;

mod common;

#[test]
fn test_check_payment_status_success() {
    let repo = common::setup_correct_payment_repository();
    let usecase = PaymentUseCase::new(&repo);

    let label = "ThisShouldWork";
    let expected_item = PaymentDetailsItem::new(label.to_string(), 99.99, "ThisShouldWorkNew".to_string(), 1);

    assert_eq!(usecase.check_payment_status(label).unwrap(), expected_item);
}

#[test]
#[should_panic(expected = "ThisIsAnError")]
fn test_check_payment_status_error() {
    let repo = common::setup_correct_payment_repository();
    let usecase = PaymentUseCase::new(&repo);

    let label = "ThisShouldNotWork";

    usecase.check_payment_status(label).unwrap();
}

#[test]
fn test_create_payment_window_success() {
    let repo = common::setup_correct_payment_repository();
    let usecase = PaymentUseCase::new(&repo);

    let payment_request_item = PaymentRequestItem::new("ThisShouldWork".to_string(), 99.99, 1);
    let expected_item = GeneratedPaymentRequestItem::new(payment_request_item.get_label().to_string(), *payment_request_item.get_amount(), "ThisShouldWorkNew".to_string());

    assert_eq!(usecase.create_payment_window(payment_request_item).unwrap(), expected_item);
}

#[test]
#[should_panic(expected = "ThisIsAnError")]
fn test_create_payment_window_error() {
    let repo = common::setup_correct_payment_repository();
    let usecase = PaymentUseCase::new(&repo);

    let payment_request_item = PaymentRequestItem::new("ThisShouldNotWork".to_string(), 99.99, 1);

    usecase.create_payment_window(payment_request_item).unwrap();
}
