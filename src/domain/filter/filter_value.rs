use super::{
    e_filter_category::EFilterCategory, filter_value_attribute::FilterValueAttribute, filter_element::FilterElement
};

#[derive(Clone)]
pub struct FilterValue {
    category: EFilterCategory,
    value: String,
    attributes: Vec<FilterValueAttribute>,
    children: Vec<FilterElement>
}

impl FilterValue {
    
    pub fn root(value: String, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::ROOT, value, attributes);
    }

    pub fn query(value: String, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::QUERY, value, attributes);
    }

    pub fn id_string(value: String, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::IDSTRING, value, attributes);
    }

    pub fn id_numeric(value: String, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::IDNUMERIC, value, attributes);
    }

    pub fn string(value: String, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::STRING, value, attributes);
    }
    
    pub fn bool(value: bool, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::BOOLEAN, value.to_string(), attributes);
    }

    pub fn i8(value: i8, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string(), attributes);
    }

    pub fn i16(value: i16, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string(), attributes);
    }

    pub fn i32(value: i32, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string(), attributes);
    }

    pub fn i64(value: i64, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string(), attributes);
    }

    pub fn i128(value: i128, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string(), attributes);
    }

    pub fn filter(value: FilterElement, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::collection(Vec::from(vec![value]));
    }

    pub fn root_collection(value: Vec<FilterElement>) -> FilterValue {
        return FilterValue::from(EFilterCategory::ROOT, String::new(), Vec::new(), value);
    }

    pub fn collection(value: Vec<FilterElement>) -> FilterValue {
        return FilterValue::from(EFilterCategory::COLLECTION, String::new(), Vec::new(), value);
    }

    fn from_value(category: EFilterCategory, value: String, attributes: Vec<FilterValueAttribute>) -> FilterValue {
        return FilterValue::from(category, value, attributes, Vec::new());
    }

    pub fn from(category: EFilterCategory, value: String, attributes: Vec<FilterValueAttribute>, children: Vec<FilterElement>) -> FilterValue {
        return FilterValue {
            category,
            value,
            attributes,
            children
        };
    }

    pub fn category(&self) -> EFilterCategory {
        return self.category.clone();
    }

    pub fn value(&self) -> String {
        return self.value.clone();
    }

    pub fn attributes(&self) -> Vec<FilterValueAttribute> {
        return self.attributes.clone();
    }

    pub fn children(&self) -> Vec<FilterElement> {
        return self.children.clone();
    }

}