use crate::{commons::exception::connect_exception::ConnectException, domain::filter::data_base_query::DataBaseQuery, infrastructure::repository::i_db_repository::IDBRepository};

#[derive(Clone)]
pub struct Service<T: IDBRepository> {
    repository: T,
}

impl <T: IDBRepository> Service<T> {

    pub fn from(repository: T) -> Service<T> {
        Service { repository }
    }

    pub async fn list_data_bases(&self) -> Result<Vec<String>, ConnectException> {
        return self.repository.list_data_bases().await;
    }

    pub async fn list_collections(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.list_collections(query).await;
    }

    pub async fn find(&self, query: DataBaseQuery) -> Result<Option<String>, ConnectException> {
        return self.repository.find(query).await;
    }

    pub async fn find_all_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_all_lite(query).await;
    }

    pub async fn find_all(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_all(query).await;
    }

    pub async fn insert(&self, query: DataBaseQuery, value:String) -> Result<String,ConnectException>{
        return self.repository.insert(query, value).await;
    }

}