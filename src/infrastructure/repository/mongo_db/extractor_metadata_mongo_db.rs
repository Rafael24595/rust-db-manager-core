use std::time::Duration;

use chrono::Local;
use mongodb::bson::{Bson, Document};

use crate::{commons::exception::connect_exception::ConnectException, domain::table::table_data_group::TableDataGroup};

pub(crate) struct ExtractorMetadataMongoDb {
}

impl ExtractorMetadataMongoDb {
    
    pub(crate) fn from_db(server_info: &Document) -> Result<Vec<TableDataGroup>, ConnectException> {
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        metadata.push(Self::metadata_general(server_info)?);
        metadata.push(Self::metadata_connection(server_info)?);
        metadata.push(Self::metadata_lock(server_info)?);
        metadata.push(Self::metadata_operation(server_info)?);

        Ok(metadata)
    }

    fn metadata_general(server_info: &Document) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(0, String::from("general"));

        let n_timestamp = server_info.get("uptimeMillis")
            .unwrap_or(&Bson::String(String::from("0")))
            .to_string().parse::<i64>()
            .unwrap_or_default()
            .try_into()
            .unwrap_or_default();

        let now = Local::now();
        let secs = Duration::from_millis(n_timestamp);
        let timestamp = now - secs;

        //let formatted_date = dt.format("%a %b %d %Y %H:%M:%S GMT%:z (%Z)").to_string();
        let formatted_date = timestamp.to_string();

        let duration = Local::now().signed_duration_since(timestamp);

        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        let uptime = format!("{}:{}:{}", hours, minutes, seconds);

        group.push(
            String::from("Hostname"),
            server_info.get("host").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Version"),
            server_info.get("version").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Started"),
            formatted_date
        );
        group.push(
            String::from("Uptime"),
           uptime
        );

        Ok(group)
    }

    fn metadata_connection(server_info: &Document) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(1, String::from("connection"));

        let o_connections = server_info.get("connections");
        if o_connections.is_none() {
            return Ok(group);
        }

        let r_connections = o_connections.unwrap().as_document();
        if r_connections.is_none() {
            return Ok(group);
        }

        let connections = r_connections.unwrap();

        group.push(
            String::from("Current"),
            connections.get("current").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Available"),
            connections.get("available").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Rejected"),
            connections.get("rejected").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Active"),
            connections.get("active").unwrap_or(&Bson::String(String::new())).to_string()
        );

        Ok(group)
    }

    fn metadata_lock(server_info: &Document) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(2, String::from("global_lock"));

        let o_lock = server_info.get("globalLock");
        if o_lock.is_none() {
            return Ok(group);
        }

        let r_lock = o_lock.unwrap().as_document();
        if r_lock.is_none() {
            return Ok(group);
        }

        let lock = r_lock.unwrap();

        let o_queue = lock.get("currentQueue");
        if o_queue.is_none() {
            return Ok(group);
        }

        let r_queue = o_queue.unwrap().as_document();
        if r_queue.is_none() {
            return Ok(group);
        }

        let queue = r_queue.unwrap();

        let o_active = lock.get("currentQueue");
        if o_active.is_none() {
            return Ok(group);
        }

        let r_active = o_active.unwrap().as_document();
        if r_active.is_none() {
            return Ok(group);
        }

        let active = r_active.unwrap();

        group.push(
            String::from("Active Clients"),
            active.get("total").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Queued Operations"),
            queue.get("total").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Clients Reading"),
            active.get("readers").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Clients Writing"),
            active.get("writers").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Read Lock Queue"),
            queue.get("readers").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Write Lock Queue"),
            queue.get("writers").unwrap_or(&Bson::String(String::new())).to_string()
        );

        Ok(group)
    }

    fn metadata_operation(server_info: &Document) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(3, String::from("operation"));

        let o_connections = server_info.get("opcounters");
        if o_connections.is_none() {
            return Ok(group);
        }

        let r_connections = o_connections.unwrap().as_document();
        if r_connections.is_none() {
            return Ok(group);
        }

        let connections = r_connections.unwrap();

        group.push(
            String::from("Total Inserts"),
            connections.get("insert").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Total Queries"),
            connections.get("query").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Total Updates"),
            connections.get("update").unwrap_or(&Bson::String(String::new())).to_string()
        );
        group.push(
            String::from("Total Deletes"),
            connections.get("delete").unwrap_or(&Bson::String(String::new())).to_string()
        );

        Ok(group)
    }

    pub(crate) fn from_collection(collection_info: Document) -> Result<Vec<TableDataGroup>, ConnectException> {
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        metadata.push(Self::_from_collections(vec![collection_info])?);
        Ok(metadata)
    }

    pub(crate) fn from_collections(collections_info: Vec<Document>) -> Result<Vec<TableDataGroup>, ConnectException> {
        let collections = collections_info.len();
        let mut group = Self::_from_collections(collections_info)?;
        group.push(String::from("Collections"), collections.to_string());
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        metadata.push(group);
        Ok(metadata)
    }

    fn _from_collections(collections_info: Vec<Document>) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(0, String::from("collection"));

        let mut count = 0;
        let mut size = 0;
        let mut storage_size = 0;
        let mut avg_obj_size = 0;
        let mut nindexes = 0;
        let mut total_index_size = 0;
        let mut total_size = 0;
        let mut index_sizes = 0;

        for collection_info in collections_info {
            count = count + collection_info.get("count").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            size =  size + collection_info.get("size").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            storage_size  = storage_size + collection_info.get("storageSize").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            avg_obj_size = avg_obj_size + collection_info.get("avgObjSize").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            nindexes = nindexes + collection_info.get("nindexes").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            total_index_size = total_index_size + collection_info.get("totalIndexSize").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            total_size = total_size + collection_info.get("totalSize").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
            index_sizes = index_sizes + collection_info.get("indexSizes").unwrap_or(&Bson::String(String::new())).to_string().parse::<i64>().unwrap_or_default();
        }

        group.push(String::from("Documents"), count.to_string());
        group.push(String::from("Data size"), format!("{:?} Bytes", size));
        group.push(String::from("Storage size"), format!("{:?} Bytes", storage_size));
        group.push(String::from("Average Object size"), format!("{:?} Bytes", avg_obj_size));
        group.push(String::from("Indexes Count"), nindexes.to_string());
        group.push(String::from("Index size"),format!("{:?} Bytes", total_index_size));
        group.push(String::from("Total Size"), total_size.to_string());
        group.push(String::from("Indexes"), index_sizes.to_string());

        Ok(group)
    }

}