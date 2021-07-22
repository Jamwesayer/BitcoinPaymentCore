use crate::business::model::Store;

pub struct StoreEntity {
    name: String,
    address: String,
    wallet_address: String
}

impl StoreEntity {
    pub fn new(name: String, address: String, wallet_address: String) -> Self {
        Self {
            name: name,
            address: address,
            wallet_address: wallet_address
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_address(&self) -> &str {
        self.address.as_str()
    }

    pub fn get_wallet_address(&self) -> &str {
        self.wallet_address.as_str()
    }
    pub fn map_to_entity(model: &Store) -> Self {
        Self {
            name: model.get_name().to_string(),
            address: model.get_address().to_string(),
            wallet_address: model.get_wallet_address().to_string()
        }
    }
    pub fn map_to_business(&self) -> Store {
        Store::new(self.get_name().to_string(), self.get_address().to_string(), self.get_wallet_address().to_string())
    }
}
