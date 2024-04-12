use super::{
    e_filter_category::EFilterCategory, 
    filter_element::FilterElement
};

#[derive(Clone)]
pub struct FilterValue {
    category: EFilterCategory,
    value: String,
    children: Vec<FilterElement>
}

impl FilterValue {
    
    pub fn from_string(value: String) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::STRING, value);
    }
    
    pub fn from_bool(value: bool) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::BOOLEAN, value.to_string());
    }

    pub fn from_i8(value: i8) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string());
    }

    pub fn from_i16(value: i16) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string());
    }

    pub fn from_i32(value: i32) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string());
    }

    pub fn from_i64(value: i64) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string());
    }

    pub fn from_i128(value: i128) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::NUMERIC, value.to_string());
    }

    pub fn from_filter(value: FilterElement) -> FilterValue {
        return FilterValue::from_collection(Vec::from(vec![value]));
    }

    pub fn from_collection(value: Vec<FilterElement>) -> FilterValue {
        return FilterValue::from(EFilterCategory::COLLECTION, String::new(), value);
    }

    fn from_value(category: EFilterCategory, value: String) -> FilterValue {
        return FilterValue::from(EFilterCategory::COLLECTION, value, Vec::new());
    }

    fn from(category: EFilterCategory, value: String, children: Vec<FilterElement>) -> FilterValue {
        return FilterValue {
            category,
            value,
            children
        };
    }

}