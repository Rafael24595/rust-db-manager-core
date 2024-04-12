use crate::domain::filter::data_base_query::DataBaseQuery;

pub(crate) trait IDBRepository {
    //TODO: Replace bytes vector returns with specific entities.
    fn status(self) -> Vec<u8>;
    fn info(self) -> Vec<u8>;
    fn find(self, query: DataBaseQuery) -> Vec<u8>;
    fn find_all(self, query: DataBaseQuery) -> Vec<String>;
    fn insert(self, query: DataBaseQuery, value: String) -> Vec<u8>;
    fn update(self, query: DataBaseQuery, value: String) -> Vec<u8>;
    fn delete(self, query: DataBaseQuery) -> Vec<u8>;
}