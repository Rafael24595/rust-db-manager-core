#[derive(Clone, PartialEq)]
pub enum EFilterCategory {
    IDSTRING,
    IDNUMERIC,
    QUERY,
    STRING,
    BOOLEAN,
    NUMERIC,
    COLLECTION,
    ROOT,
}