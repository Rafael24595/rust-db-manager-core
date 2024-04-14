use super::{e_filter_category::EFilterCategory, filter_value::FilterValue};

#[derive(Clone)]
pub struct FilterElement {
    key: String,
    value: FilterValue,
    negation: bool,
}

impl FilterElement {
    
    pub fn new() -> FilterElement {
        let f_value = FilterValue::from_root(String::new());
        return FilterElement::from(String::new(), f_value, false);
    }

    pub fn from_query(value: String) -> FilterElement {
        let f_value = FilterValue::from_query(value);
        return FilterElement::from(String::new(), f_value, false);
    }

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
        let mut collection = Vec::new();
        if self.value.category() != EFilterCategory::ROOT {
            collection.push(self.clone());
        }
        collection.push(filter);

        let value = FilterValue::from_collection(collection);
        return FilterElement::from_value(self.key.clone(), value);
    }

    pub fn negate(&mut self) -> &mut FilterElement {
        self.negation = true;
        return self;
    }

    pub fn negate_ref(&mut self) -> FilterElement {
        return self.negate().as_ref();
    }

    pub fn affirmate(&mut self) -> &mut FilterElement {
        self.negation = false;
        return self;
    }

    pub fn affirmate_ref(&mut self) -> FilterElement {
        return self.affirmate().as_ref();
    }

    pub fn field(&self) -> String {
        return self.key.clone();
    }

    pub fn value(&self) -> &FilterValue {
        return &self.value;
    }

    pub fn is_negate(&self) -> bool {
        return self.negation;
    }

    pub fn as_ref(&self) -> FilterElement {
        return self.clone();
    } 

}