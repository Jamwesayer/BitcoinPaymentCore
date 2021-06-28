use crate::business::irepository::IPaymentRepository;
use crate::PaymentRequestItem;

pub struct PaymentUseCase<'a> {
    payment_repository: &'a dyn IPaymentRepository
}

impl<'a> PaymentUseCase<'a> {
    pub fn createPaymentWindow(&self, payment_request_item: PaymentRequestItem) {
        let test = self.payment_repository.create_payment_window(payment_request_item.map_to_business());
        println!("{:?}", test);
    }

    pub fn new(_payment_repository: &'a dyn IPaymentRepository) -> Self {
        Self {
            payment_repository: _payment_repository
        }
    }

}
