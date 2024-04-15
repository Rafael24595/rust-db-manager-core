use super::{e_filter_category::EFilterCategory, filter_value::FilterValue};

#[derive(Clone)]
pub struct FilterElement {
    key: String,
    value: FilterValue,
    direction: bool,
    negation: bool,
}

impl FilterElement {
    
    pub fn new() -> FilterElement {
        let f_value = FilterValue::from_root(String::new());
        return FilterElement::from(String::new(), f_value, true, false);
    }

    pub fn from_query(value: String) -> FilterElement {
        let f_value = FilterValue::from_query(value);
        return FilterElement::from(String::new(), f_value, true, false);
    }

    pub fn from_id(key: String, value: String) -> FilterElement {
        let f_value = FilterValue::from_id(value);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from_string(key: String, value: String) -> FilterElement {
        let f_value = FilterValue::from_string(value);
        return FilterElement::from(key, f_value, true, false);
    }
    
    pub fn from_bool(key: String, value: bool) -> FilterElement {
        let f_value = FilterValue::from_bool(value);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from_i8(key: String, value: i8) -> FilterElement {
        let f_value = FilterValue::from_i8(value);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from_i16(key: String, value: i16) -> FilterElement {
        let f_value = FilterValue::from_i16(value);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from_i32(key: String, value: i32) -> FilterElement {
        let f_value = FilterValue::from_i32(value);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from_i64(key: String, value: i64) -> FilterElement {
        let f_value = FilterValue::from_i64(value);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from_i128(key: String, value: i128) -> FilterElement {
        let f_value = FilterValue::from_i128(value);
        return FilterElement::from(key, f_value, true, false);
    }

    fn from_value(key: String, value: FilterValue) -> FilterElement {
        return FilterElement::from(key, value, true, false);
    }

    fn from(key: String, value: FilterValue, direction: bool, negation: bool) -> FilterElement {
        return FilterElement {
            key,
            value,
            direction,
            negation
        };
    }

    pub fn from_id_chain(chain: String) -> FilterElement {
        let keys: Vec<&str> = chain.split('#').collect();

        let mut filter = FilterElement::new();

        for key in keys {
            let entry: Vec<&str> = key.split('=').collect();
            if entry.len() > 1 {
                let code = String::from(*entry.get(0).unwrap());
                let value = String::from(*entry.get(1).unwrap());
                filter.push(FilterElement::from_id(code, value));
            }
        }

        return filter;
    }

    pub fn from_id_chain_collection(keys: Vec<String>) -> FilterElement {
        let mut filter = FilterElement::new();

        for key in keys {
            let child = FilterElement::from_id_chain(key).as_or_ref();
            filter.push(child);
        }

        return filter;
    }

}

impl FilterElement {
    
    pub fn push(&mut self, mut filter: FilterElement) -> &Self {
        let mut collection = Vec::new();

        if filter.value.category() == EFilterCategory::ROOT {
            filter.value = FilterValue::from_collection(filter.value.children());
        }

        collection.push(filter);

        let value;
        if self.value.category() == EFilterCategory::ROOT {
            collection.append(&mut self.value.children());
            value = FilterValue::from_root_collection(collection);
        } else {
            collection.push(self.clone());
            value = FilterValue::from_collection(collection);
        }

        self.value = value;
        
        return self;
    }

    pub fn as_and(&mut self) -> &mut FilterElement {
        self.direction = true;
        return self;
    }

    pub fn as_and_ref(&mut self) -> FilterElement {
        return self.as_and().as_ref();
    }

    pub fn as_or(&mut self) -> &mut FilterElement {
        self.direction = false;
        return self;
    }

    pub fn as_or_ref(&mut self) -> FilterElement {
        return self.as_or().as_ref();
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

    pub fn is_or(&self) -> bool {
        return !self.direction;
    }

    pub fn as_ref(&self) -> FilterElement {
        return self.clone();
    } 

}