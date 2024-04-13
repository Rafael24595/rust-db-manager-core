use crate::domain::filter::{e_filter_category::EFilterCategory, filter_element::FilterElement, filter_value::FilterValue};

pub struct QueryItems {
    fields: Vec<String>,
    queries: Vec<String>
}

impl FilterElement {
    
    pub fn as_mongo_agregate(&self) -> String {
        let mut registry = QueryItems {fields: Vec::new(), queries: Vec::new()};
        registry = self._as_mongo_agregate(registry);

        let mut result = Vec::<String>::new();

        if !registry.fields.is_empty() {
            let match_string = format!("{{ $match: {{ $and: [ {} ] }} }}", registry.fields.join(", "));
            result.push(match_string);
        }

        if !registry.queries.is_empty() {
            let query_string = registry.queries.join(", ");
            result.push(query_string);
        }

        return format!("[ {} ]", result.join(", "));
    }

    fn _as_mongo_agregate(&self, mut registry: QueryItems) -> QueryItems {
        let f_value = self.value();
        let field = self.field();

        let result = f_value.as_mongo_agregate(registry);
        let mut value = result.0;
        registry = result.1;

        if f_value.category() == EFilterCategory::COLLECTION {
            return registry;    
        }

        if f_value.category() == EFilterCategory::QUERY {
            registry.queries.push(value);
            return registry;    
        }

        if self.is_negate() {
            value = format!("{{ $not: {{ $eq: {} }} }}", value);
        }

        let query = format!("{{ {}: {} }}", field, value);
        registry.fields.push(query);
        return registry;
    }

}

impl FilterValue {
    
    pub fn as_mongo_agregate(&self, registry: QueryItems) -> (String, QueryItems) {
        let value = self.value();
        match self.category() {
            EFilterCategory::ID => (value, registry),
            EFilterCategory::QUERY => (value, registry),
            EFilterCategory::STRING => (format!("\"{}\"", value), registry),
            EFilterCategory::BOOLEAN => (value, registry),
            EFilterCategory::NUMERIC => (value, registry),
            EFilterCategory::COLLECTION => (value, self.collection_as_mongo_agregate(registry)),
            EFilterCategory::ROOT => (value, registry),
        }
    }

    fn collection_as_mongo_agregate(&self, mut registry: QueryItems) -> QueryItems {
        for child in self.children() {
            registry = child._as_mongo_agregate(registry);
        }
        return registry;
    }

}