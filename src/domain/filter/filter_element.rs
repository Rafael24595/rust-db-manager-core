use super::filter_value::FilterValue;

#[derive(Clone)]
pub struct FilterElement {
    key: String,
    value: FilterValue,
    negation: bool,
}

impl FilterElement {
    
    pub fn from_string(key: String, value: String) -> FilterElement {
        let f_value = FilterValue::from_string(value);
        return FilterElement::from(key, f_value, false);
    }
    
    pub fn from_bool(key: String, value: bool) -> FilterElement {
        let f_value = FilterValue::from_bool(value);
        return FilterElement::from(key, f_value, false);
    }

    pub fn from_i8(key: String, value: i8) -> FilterElement {
        let f_value = FilterValue::from_i8(value);
        return FilterElement::from(key, f_value, false);
    }

    pub fn from_i16(key: String, value: i16) -> FilterElement {
        let f_value = FilterValue::from_i16(value);
        return FilterElement::from(key, f_value, false);
    }

    pub fn from_i32(key: String, value: i32) -> FilterElement {
        let f_value = FilterValue::from_i32(value);
        return FilterElement::from(key, f_value, false);
    }

    pub fn from_i64(key: String, value: i64) -> FilterElement {
        let f_value = FilterValue::from_i64(value);
        return FilterElement::from(key, f_value, false);
    }

    pub fn from_i128(key: String, value: i128) -> FilterElement {
        let f_value = FilterValue::from_i128(value);
        return FilterElement::from(key, f_value, false);
    }

    fn from_value(key: String, value: FilterValue) -> FilterElement {
        return FilterElement::from(key, value, false);
    }

    fn from(key: String, value: FilterValue, negation: bool) -> FilterElement {
        return FilterElement {
            key,
            value,
            negation
        };
    }

}

impl FilterElement {
    
    pub fn push(&self, filter: FilterElement) -> FilterElement {
        let collection = Vec::from(vec![self.clone(), filter]);
        let value = FilterValue::from_collection(collection);
        let element = FilterElement::from_value(self.key.clone(), value);
        return element;
    }

    pub fn negate(& mut self) {
        self.negation = true;
    }

    pub fn affirmate(& mut self) {
        self.negation = false;
    }

}