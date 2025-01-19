use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{e_json_type::EJSONType, filter::document_query::DocumentQuery},
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

        //TODO: Add sort logic.

        Ok(format!("{};", sql.join(" ")))
    }

    pub fn make_where_clause(&self) -> Result<Vec<String>, ConnectException> {
        let filter = self.filter();
        if filter.is_none() {
            return Ok(Vec::new());
        }

        let filter = filter.unwrap();

        let mut conditions = Vec::new();
        for (i, field) in filter.value().children().iter().enumerate() {
            let mut header = "WHERE";
            if i > 0 {
                header = "AND";
            }

            let key = field.field();
            let symbol = "=";
            let value = field.value().value();

            let condition = format!("{} {} {} {}", header, key, symbol, value);
            conditions.push(condition);
        }
        Ok(conditions)
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