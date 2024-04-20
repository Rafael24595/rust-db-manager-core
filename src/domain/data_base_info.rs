pub struct DataBaseInfo {
    name: String,
    version: String,
    sw_relational: bool,
    needs_data_base: bool,
    needs_collection: bool
}

impl DataBaseInfo {
    
    pub fn new_relational(name: String, version: String, needs_data_base: bool, needs_collection: bool) -> DataBaseInfo {
        DataBaseInfo {
            name: name,
            version: version,
            sw_relational: true,
            needs_data_base: needs_data_base,
            needs_collection: needs_collection
        }
    }

    pub fn new_no_relational(name: String, version: String, needs_data_base: bool, needs_collection: bool) -> DataBaseInfo {
        DataBaseInfo {
            name: name,
            version: version,
            sw_relational: false,
            needs_data_base: needs_data_base,
            needs_collection: needs_collection
        }
    }

}