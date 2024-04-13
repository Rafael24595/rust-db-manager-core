#[derive(Clone, PartialEq)]
pub enum EFilterCategory {
    ID,
    QUERY,
    STRING,
    BOOLEAN,
    NUMERIC,
    COLLECTION,
    ROOT,
}