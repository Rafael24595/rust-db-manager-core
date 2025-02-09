use super::filter_value::FilterValue;

#[derive(Clone)]
pub struct FilterElement {
    key: String,
    value: FilterValue,
    direction: bool,
    negation: bool,
}

impl FilterElement {

    pub fn new() -> Self {
        let f_value = FilterValue::from(String::new(), String::new(), String::new(), Vec::new(), Vec::new());
        Self::from(String::new(), f_value, true, false)
    }

    pub fn from(key: String, value: FilterValue, direction: bool, negation: bool) -> FilterElement {
        return FilterElement {
            key,
            value,
            direction,
            negation
        };
    }

}

impl FilterElement {
    
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &FilterValue {
        &self.value
    }

    pub fn is_negate(&self) -> bool {
        self.negation
    }

    pub fn is_and(&self) -> bool {
        self.direction
    }

    pub fn is_or(&self) -> bool {
        !self.direction
    }

    pub fn set_value(&mut self, value: FilterValue) -> &Self {
        self.value = value;
        self
    }

    pub fn as_ref(&self) -> FilterElement {
        self.clone()
    } 

}