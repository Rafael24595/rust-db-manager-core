use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{collection::generate_collection_query::GenerateCollectionQuery, e_json_type::EJSONType, filter::document_query::DocumentQuery},
};

impl DocumentQuery {
    
    pub fn as_postres_sql(&self) -> Result<String, ConnectException> {
        let mut sql = Vec::new();
        sql.push(String::from("SELECT"));

        //TODO: Review.
        let mut fields = Vec::new();
        fields.push(String::from("row_to_json(r)"));

        let mut conditions = self.make_where_clause()?;

        let fields = fields.join(", ");
        sql.push(fields);

        let from = format!("FROM {} AS r", self.collection());
        sql.push(from);

        if conditions.len() != 0 {
            sql.append(&mut conditions);
        }

        if let Some(limit) = self.limit() {
            let limit = format!("LIMIT {}", limit);
            sql.push(limit);
        }

        if let Some(skip) = self.skip() {
            let skip = format!("OFFSET {}", skip);
            sql.push(skip);
        }

        //TODO: Add sort logic.

        Ok(format!("{};", sql.join(" ")))
    }

    pub fn make_where_clause(&self) -> Result<Vec<String>, ConnectException> {
        let filter = self.filter();
        if filter.is_none() {
            return Ok(Vec::new());
        }

        let filter = filter.as_ref().unwrap();

        let mut conditions = Vec::new();
        for (i, field) in filter.value().children().iter().enumerate() {
            let mut header = "WHERE";
            if i > 0 {
                header = "AND";
            }

            let key = field.key();
            let symbol = "=";
            let value = field.value().value();

            let condition = format!("{} {} {} {}", header, key, symbol, value);
            conditions.push(condition);
        }
        Ok(conditions)
    }

}

impl GenerateCollectionQuery {
    
    pub fn to_postgres_query(&self) -> String {
        let mut buffer = Vec::new();

        let header = format!("CREATE TABLE {} (", self.collection());
        buffer.push(header);

        let mut buffer_fields = Vec::new();
        let mut buffer_fks = Vec::new();
        for field in self.fields() {
            let name = field.value();
            let field_type = field.code();

            let mut size_status = String::new();
            if field.is_resize() {
                size_status = format!("({})", field.size());
            }

            let mut not_null_status = String::new();
            let attribute = field.attributes().iter()
                .find(|a| a.key() == "NOT_NULL")
                .map(|a| a.value().parse::<bool>().unwrap_or(false))
                .unwrap_or(false);
            if !attribute {
                not_null_status = String::from("NOT NULL");
            }

            let mut key_status = String::new();
            if field.is_key() {
                key_status = String::from("PRIMARY KEY");
            }

            let mut unique_status = String::new();
            let attribute = field.attributes().iter()
                .find(|a| a.key() == "UNIQUE")
                .map(|a| a.value().parse::<bool>().unwrap_or(false))
                .unwrap_or(false);
            if !field.is_key()  && attribute {
                unique_status = String::from("UNIQUE");
            }

            let field_sentence = format!("{} {}{} {} {} {}", name, field_type, size_status, not_null_status, key_status, unique_status);

            buffer_fields.push(field_sentence);

            if field.reference().len() > 0 {
                let reference = &field.reference()[0];
                let collection = reference.collection();
                let field = reference.field();
                let mut cascade_status = "";
                if reference.cascade() {
                    cascade_status = "ON DELETE CASCADE";
                }

                let key_sentence = format!("FOREIGN KEY  ({}) REFERENCES {}({}) {}", name, collection, field, cascade_status);

                buffer_fks.push(key_sentence);
            }
        }

        buffer_fields.append(&mut buffer_fks);
        buffer.push(buffer_fields.join(", "));
        buffer.push(String::from(");"));

        buffer.join("")
    }

}

pub(crate) fn postgres_to_json_type(postgres_type: &str) -> EJSONType {
    match postgres_type {
        "integer" | "bigint" | "smallint" | "serial" | "bigserial" => EJSONType::NUMERIC,
        "real" | "double precision" | "numeric" => EJSONType::NUMERIC,
        "boolean" => EJSONType::BOOLEAN,
        "text" | "varchar" | "char" | "character varying" | "uuid" => EJSONType::STRING,
        "date" | "timestamp" | "timestamptz" | "time" | "timetz" | "interval" => EJSONType::STRING,
        "json" | "jsonb" => EJSONType::OBJECT,
        "bytea" => EJSONType::STRING,
        ty if ty.ends_with("[]") => EJSONType::ARRAY,
        "inet" | "cidr" | "macaddr" => EJSONType::STRING,
        "hstore" => EJSONType::OBJECT,
        _ => EJSONType::STRING,
    }
}