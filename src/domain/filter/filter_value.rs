use super::{
    filter_value_attribute::FilterValueAttribute, filter_element::FilterElement
};

#[derive(Clone)]
pub struct FilterValue {
    category: String,
    json_type: String,
    value: String,
    attributes: Vec<FilterValueAttribute>,
    children: Vec<FilterElement>
}

impl FilterValue {

    pub fn from(category: String, json_type: String, value: String, attributes: Vec<FilterValueAttribute>, children: Vec<FilterElement>) -> FilterValue {
        return FilterValue {
            category,
            json_type,
            value,
            attributes,
            children
        };
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn json_type(&self) -> &str {
        &self.json_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn attributes(&self) -> &Vec<FilterValueAttribute> {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<FilterElement> {
        &self.children
    }

}