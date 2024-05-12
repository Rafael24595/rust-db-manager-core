use crate::domain::{document::document_key::DocumentKey, e_json_type::EJSONType, filter::{filter_element::FilterElement, filter_value_attribute::FilterValueAttribute}};

pub fn document_keys_to_filter_element(documents: Vec<DocumentKey>) -> FilterElement {
    let mut filter = FilterElement::new();

    for document in documents {
        match document.jtype() {
            EJSONType::STRING => {
                filter.push(FilterElement::id_string(
                    document.name(), 
                    document.value(), 
                    document.attributes().iter()
                    .map(|a| FilterValueAttribute::new(a.key(), a.value())).collect()));
            },
            EJSONType::NUMERIC => {
                filter.push(FilterElement::id_numeric(
                    document.name(), 
                    document.value(), 
                    document.attributes().iter()
                    .map(|a| FilterValueAttribute::new(a.key(), a.value())).collect()));
            },
            EJSONType::BOOLEAN => {
                //TODO: error
            },
        }
    }

    filter
}