use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>)
}