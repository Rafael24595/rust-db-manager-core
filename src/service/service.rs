use crate::{commons::exception::connect_exception::ConnectException, domain::filter::data_base_query::DataBaseQuery, infrastructure::repository::i_db_repository::IDBRepository};

#[derive(Clone)]
pub struct Service<T: IDBRepository> {
    repository: T,
}

impl <T: IDBRepository> Service<T> {

    pub fn from(repository: T) -> Service<T> {
        Service { repository }
    }

    async fn list_data_bases(self) -> Result<Vec<String>, ConnectException> {
        return self.repository.list_data_bases().await;
    }

    async fn list_collections(self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.list_collections(query).await;
    }

    async fn find(self, query: DataBaseQuery) -> Result<Vec<u8>, ConnectException> {
        return self.repository.find(query).await;
    }

}