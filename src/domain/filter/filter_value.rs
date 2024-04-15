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
    
    pub fn from_root(value: String) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::ROOT, value);
    }

    pub fn from_query(value: String) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::QUERY, value);
    }

    pub fn from_id(value: String) -> FilterValue {
        return FilterValue::from_value(EFilterCategory::ID, value);
    }

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

    pub fn from_root_collection(value: Vec<FilterElement>) -> FilterValue {
        return FilterValue::from(EFilterCategory::ROOT, String::new(), value);
    }

    pub fn from_collection(value: Vec<FilterElement>) -> FilterValue {
        return FilterValue::from(EFilterCategory::COLLECTION, String::new(), value);
    }

    fn from_value(category: EFilterCategory, value: String) -> FilterValue {
        return FilterValue::from(category, value, Vec::new());
    }

    fn from(category: EFilterCategory, value: String, children: Vec<FilterElement>) -> FilterValue {
        return FilterValue {
            category,
            value,
            children
        };
    }

    pub fn category(&self) -> EFilterCategory {
        return self.category.clone();
    }

    pub fn value(&self) -> String {
        return self.value.clone();
    }

    pub fn children(&self) -> Vec<FilterElement> {
        return self.children.clone();
    }

}