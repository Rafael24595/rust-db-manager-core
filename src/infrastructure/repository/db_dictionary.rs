use url::Url;

use crate::{commons::exception::connect_exception::ConnectException, domain::connection_data::ConnectionData};

use super::{e_db_repository::EDBRepository, i_db_repository::IDBRepository, mongo_db::mongo_db_repository::MongoDbRepository, postgres::postgres_repository::PostgresRepository};

//TODO: Cache instances.
pub async fn find(connection: &ConnectionData) -> Result<Box<dyn IDBRepository>, ConnectException> {
    match connection.category() {
        EDBRepository::Identify => panic!("Invalid scheme '{}'.", connection.category().to_string()),
        EDBRepository::MongoDB => Ok(MongoDbRepository::new(connection).await?),
        EDBRepository::Postgres => Ok(PostgresRepository::new(connection).await?)
    }
}
