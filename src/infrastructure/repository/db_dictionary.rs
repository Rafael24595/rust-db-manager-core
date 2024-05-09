use crate::{commons::exception::connect_exception::ConnectException, domain::connection_data::ConnectionData};

use super::{e_db_repository::EDBRepository, i_db_repository::IDBRepository, mongo_db::mongo_db_repository::MongoDbRepository};

pub async fn find(connection: &ConnectionData) -> Result<impl IDBRepository, ConnectException>  {
    match connection.category() {
        EDBRepository::MongoDB => Ok(MongoDbRepository::new(connection).await?)
    }
}