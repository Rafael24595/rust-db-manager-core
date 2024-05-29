use super::{e_filter_category::EFilterCategory, filter_value_attribute::FilterValueAttribute, filter_value::FilterValue};

#[derive(Clone)]
pub struct FilterElement {
    key: String,
    value: FilterValue,
    direction: bool,
    negation: bool,
}

impl FilterElement {
    
    pub fn new() -> FilterElement {
        let f_value = FilterValue::root(String::new(), Vec::new());
        return FilterElement::from(String::new(), f_value, true, false);
    }

    pub fn query(value: String, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::query(value, attributes);
        return FilterElement::from(String::new(), f_value, true, false);
    }

    pub fn id_string(key: String, value: String, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::id_string(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn id_numeric(key: String, value: String, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::id_numeric(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn string(key: String, value: String, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::string(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }
    
    pub fn bool(key: String, value: bool, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::bool(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn i8(key: String, value: i8, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::i8(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn i16(key: String, value: i16, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::i16(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn i32(key: String, value: i32, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::i32(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn i64(key: String, value: i64, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::i64(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn i128(key: String, value: i128, attributes: Vec<FilterValueAttribute>) -> FilterElement {
        let f_value = FilterValue::i128(value, attributes);
        return FilterElement::from(key, f_value, true, false);
    }

    pub fn from(key: String, value: FilterValue, direction: bool, negation: bool) -> FilterElement {
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
                filter.push(FilterElement::id_string(code, value, Vec::new()));
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
            filter.value = FilterValue::collection(filter.value.children());
        }

        collection.push(filter);

        let value;
        if self.value.category() == EFilterCategory::ROOT {
            collection.append(&mut self.value.children());
            value = FilterValue::root_collection(collection);
        } else {
            collection.push(self.clone());
            value = FilterValue::collection(collection);
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