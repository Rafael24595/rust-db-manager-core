use super::{
    filter_value_attribute::FilterValueAttribute, filter_element::FilterElement
};

#[derive(Clone)]
pub struct FilterValue {
    category: String,
    value: String,
    attributes: Vec<FilterValueAttribute>,
    children: Vec<FilterElement>
}

impl FilterValue {

    pub fn from(category: String, value: String, attributes: Vec<FilterValueAttribute>, children: Vec<FilterElement>) -> FilterValue {
        return FilterValue {
            category,
            value,
            attributes,
            children
        };
    }

    pub fn category(&self) -> &str {
        &self.category
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