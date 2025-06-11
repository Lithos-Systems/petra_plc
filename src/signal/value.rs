use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SignalValue {
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
}

impl SignalValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SignalValue::Bool(b) => Some(*b),
            SignalValue::Int(i) => Some(*i != 0),
            _ => None,
        }
    }
    
    pub fn as_int(&self) -> Option<i32> {
        match self {
            SignalValue::Int(i) => Some(*i),
            SignalValue::Bool(b) => Some(if *b { 1 } else { 0 }),
            SignalValue::Float(f) => Some(*f as i32),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            SignalValue::Float(f) => Some(*f),
            SignalValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    pub fn type_name(&self) -> &'static str {
        match self {
            SignalValue::Bool(_) => "bool",
            SignalValue::Int(_) => "int",
            SignalValue::Float(_) => "float",
            SignalValue::String(_) => "string",
        }
    }
}
