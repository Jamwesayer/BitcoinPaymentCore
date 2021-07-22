use crate::business::irepository::IStoreRepository;
use crate::data::entity::store::StoreEntity;
use crate::data::idatasource::{IStoreDatabaseDatasource, StoreDatabase};
use crate::business::model::Store;

pub struct StoreRepository {
    store_database_datasource: Box<dyn IStoreDatabaseDatasource>
}

impl StoreRepository {
    pub fn new(store_database_datasource: Box<dyn IStoreDatabaseDatasource>) -> Self {
        Self {
            store_database_datasource: store_database_datasource,
        }
    }
}

impl IStoreRepository for StoreRepository {
    fn register_store(&self, store_item: Store) -> Result<Store, String> {
        let store_entity = self.store_database_datasource.register_store(StoreEntity::map_to_entity(&store_item))?;
        Ok(store_entity.map_to_business())
    }

    fn login(&self) -> Result<(), String> {
        todo!()
    }
}

impl Default for StoreRepository {
    fn default() -> Self {
        Self {
            store_database_datasource: Box::new(StoreDatabase::default())
        }
    }
}