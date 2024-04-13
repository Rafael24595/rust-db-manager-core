use async_trait::async_trait;

use crate::{commons::exception::connect_exception::ConnectException, domain::filter::data_base_query::DataBaseQuery};

#[async_trait]
pub trait IDBRepository {
    //TODO: Replace bytes vector returns with specific entities.
    async fn status(self) -> Result<(), ConnectException>;
    fn info(self) -> Vec<u8>;
    fn find(self, query: DataBaseQuery) -> Vec<u8>;
    fn find_all(self, query: DataBaseQuery) -> Vec<String>;
    fn insert(self, query: DataBaseQuery, value: String) -> Vec<u8>;
    fn update(self, query: DataBaseQuery, value: String) -> Vec<u8>;
    fn delete(self, query: DataBaseQuery) -> Vec<u8>;
}