use crate::business::irepository::IStoreRepository;
use crate::data::repository::store::StoreRepository;
use crate::presentation::item::*;

pub struct StoreUseCase {
    store_repository: Box<dyn IStoreRepository>
}

impl StoreUseCase {

    pub fn register_store(&self, register_store_item: StoreItem) -> Result<StoreItem, String> {
        let store_model = self.store_repository.register_store(register_store_item.map_to_business())?;
        Ok(store_model.map_to_presentation())
    }

    pub fn login(&self) {
        
    }

}