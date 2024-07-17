use mongodb::{bson::{doc, Bson, Document}, options::IndexOptions, Collection, IndexModel};

use crate::{commons::exception::connect_exception::ConnectException, domain::action::generate::action::Action};

pub const ACTION_INDEXES_NEW: &str = "INDEXES_NEW";
pub const FORM_FIELDS: &str = "FIELDS";
pub const FIELD_FIELD: &str = "FIELD";
pub const FIELD_DIRECTION: &str = "DIRECTION";
pub const FORM_ATTRIBUTES: &str = "ATTRIBUTES";
pub const FIELD_NAME: &str = "NAME";
pub const FIELD_UNIQUE: &str = "UNIQUE";

pub const ACTION_INDEXES_DELETE: &str = "INDEXES_DELETE";
pub const FORM_INDEXED: &str = "INDEXED";
pub const FIELD_INDEXED: &str = "INDEXED";

pub(crate) async  fn execute_collection_action(collection: Collection<Document>, action: &Action) -> Result<String, ConnectException> {
    match action.action().as_str() {
        ACTION_INDEXES_NEW => create_indexes(collection, action).await,
        ACTION_INDEXES_DELETE => delete_indexes(collection, action).await,
        _ => Err(ConnectException::new(String::from("Action not recognized.")))
    }
}

async fn create_indexes(collection: Collection<Document>, action: &Action) -> Result<String, ConnectException> {
    let index = IndexModel::builder()
        .keys(create_indexes_keys(action).await?)
        .options(create_indexes_options(action).await?)
        .build();

    if let Err(result) = collection.create_index(index, None).await {
        let exception = ConnectException::new(result.to_string());
        return Err(exception);
    }

    Ok(String::from("Indexes created successfully."))
}

async fn create_indexes_keys(action: &Action) -> Result<Document, ConnectException> {
    let mut keys = doc! {};

    let o_form_fields = action.find_form(String::from(FORM_FIELDS));
    if o_form_fields.is_none() {
        return Err(ConnectException::new(String::from("Form data not found.")));
    }

    let form_fields = o_form_fields.unwrap();
    for fields in form_fields.fields() {
        let o_field = fields.iter().find(|f| f.code() == String::from(FIELD_FIELD));
        let o_direction = fields.iter().find(|f| f.code() == String::from(FIELD_DIRECTION));
        if o_field.is_none() {
            continue;
        }

        let field = o_field.unwrap().value();

        let mut direction = 1;
        if let Some(result) = o_direction {
            direction = result.value()
                .parse::<i32>()
                .unwrap_or(1);
        }

        keys.insert(field, Bson::Int32(direction));
    }

    Ok(keys)
}

async fn create_indexes_options(action: &Action) -> Result<IndexOptions, ConnectException> {
    let o_form_attributes = action.find_form(String::from(FORM_ATTRIBUTES));
    if o_form_attributes.is_none() {
        return Ok(IndexOptions::builder().build());
    }

    let form_attributes = o_form_attributes.unwrap();

    let mut name = None;
    if let Some(values) = form_attributes.find_fields(String::from(FIELD_NAME)).first() {
        name = Some(values.value());
    }

    let mut unique = true;
    if let Some(values) = form_attributes.find_fields(String::from(FIELD_UNIQUE)).first() {
        unique = values.value().parse::<bool>().unwrap_or(true);
    }

    Ok(IndexOptions::builder()
        .name(name)
        .unique(unique)
        .build())
}

async fn delete_indexes(collection: Collection<Document>, action: &Action) -> Result<String, ConnectException> {
    let form = action.find_form(String::from(FORM_INDEXED));
    if form.is_none() {
        return Err(ConnectException::new(String::from("Form data not found.")));
    }

    let indexes = &form.unwrap().find_fields(String::from(FIELD_INDEXED));

    let mut errors = Vec::new();
    for index in indexes {
        let value = index.value();

        if let Err(error) = collection.drop_index(value.clone(), None).await {
            errors.push(value + ": " + &error.to_string());
        }
    }

    if indexes.len() > 0 && errors.len() == 0 {
        return Ok(String::from("All indexes removed."))
    }

    if errors.len() > 0 {
        let message = String::from("Some indexes cannot be removed: \n") + &errors.join("");
        return Ok(message)
    }

    Ok(String::from("No indexes removed."))
}