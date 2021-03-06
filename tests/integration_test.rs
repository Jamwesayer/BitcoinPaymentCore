use payment_core::presentation::item::*;
use payment_core::business::usecase::payment::PaymentUseCase;
use payment_core::business::usecase::transaction::TransactionUseCase;
use payment_core::data::entity::transaction::*;

mod common;
use chrono::naive::*;

#[test]
fn test_check_payment_status_success() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let label = "ThisShouldWork";
    let search_item = PaymentWindowSearchItem::new(label.to_string(), 1);
    let expected_item = PaymentDetailsItem::new(label.to_string(), 99.99, "ThisShouldWorkNew".to_string(), 1);

    assert_eq!(usecase.check_payment_status(search_item).unwrap(), expected_item);
}

#[test]
#[should_panic(expected = "ThisIsAnError")]
fn test_check_payment_status_error() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let label = "ThisShouldNotWork";
    let search_item = PaymentWindowSearchItem::new(label.to_string(), 1);

    usecase.check_payment_status(search_item).unwrap();
}

#[test]
fn test_create_payment_window_success() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let payment_request_item = PaymentRequestItem::new("ThisShouldWork".to_string(), 99.99, 1);
    let expected_item = GeneratedPaymentRequestItem::new(payment_request_item.get_label().to_string(), *payment_request_item.get_amount(), "ThisShouldWorkNew".to_string());

    assert_eq!(usecase.create_payment_window(&payment_request_item).unwrap(), expected_item);
}

#[test]
#[should_panic(expected = "ThisIsAnError")]
fn test_create_payment_window_error() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let payment_request_item = PaymentRequestItem::new("ThisShouldNotWork".to_string(), 99.99, 1);

    usecase.create_payment_window(&payment_request_item).unwrap();
}

#[test]
fn test_suspend_payment_window_success() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let label = "ThisShouldWork";
    let search_item = PaymentWindowSearchItem::new(label.to_string(), 1);
    let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);

    assert_eq!(usecase.suspend_payment_window(search_item).unwrap().0, "De betaling is geanulleerd".to_string());
}

#[test]
#[should_panic(expected = "ThisIsAnError")]
fn test_suspend_payment_window_fail() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let label = "ThisShouldNotWork";
    let search_item = PaymentWindowSearchItem::new(label.to_string(), 1);

    usecase.suspend_payment_window(search_item).unwrap();
}

#[test]
fn test_suspend_payment_window_with_refund_success() {
    let repo0 = common::setup_correct_payment_repository();
    let repo1 = common::setup_correct_transaction_repository();
    let usecase = PaymentUseCase::new(Box::new(repo0), Box::new(repo1));

    let label = "ThisShouldWork";
    let search_item = PaymentWindowSearchItem::new(label.to_string(), 1);
    let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
    let expected_transaction = TransactionItem::new(10.0, "hash".to_string(), "origin".to_string(), dt, 1, 1);

    assert_eq!(usecase.suspend_payment_window(search_item).unwrap().1[0], expected_transaction);
}

// --------------------------------------------------------- transaction
#[test]
fn test_gather_list_of_items_specific_to_store_success() {
    let repo = common::setup_correct_transaction_repository();
    let usecase = TransactionUseCase::new(Box::new(repo));

    let mut transactions = Vec::new();
    for n in 1 .. 10 {
        transactions.push(TransactionItem::new(10.0, "test".to_string(), "origin".to_string(), NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11), n, n));
    }

    assert_eq!(usecase.get_all_transactions(&1).unwrap(), transactions);
}

#[test]
#[should_panic(expected = "ThisIsAnError")]
fn test_gather_list_of_items_specific_to_store_fail() {
    let repo = common::setup_correct_transaction_repository();
    let usecase = TransactionUseCase::new(Box::new(repo));

    usecase.get_all_transactions(&0).unwrap();
}

#[test]
fn test_get_transaction_by_transaction_id_success() {
    let repo = common::setup_correct_transaction_repository();
    let usecase = TransactionUseCase::new(Box::new(repo));

    let item = TransactionItem::new(10.0, "hash".to_string(), "origin".to_string(), NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11), 1, 1);

    assert_eq!(usecase.get_transaction_by_transaction_id("ThisShouldWork").unwrap(), item);
}

#[test]
#[should_panic(expected = "De transactie met transactie nummer ThisShouldNotWork bestaat niet")]
fn test_get_transaction_by_transaction_id_fail() {
    let repo = common::setup_correct_transaction_repository();
    let usecase = TransactionUseCase::new(Box::new(repo));

    usecase.get_transaction_by_transaction_id("ThisShouldNotWork").unwrap();
}
