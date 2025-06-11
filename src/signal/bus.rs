use dashmap::DashMap;
use std::sync::Arc;
use crate::{PlcError, Result};
use super::SignalValue;

#[derive(Clone)]
pub struct SignalBus {
    signals: Arc<DashMap<String, SignalValue>>,
}

impl SignalBus {
    pub fn new() -> Self {
        Self {
            signals: Arc::new(DashMap::new()),
        }
    }
    
    pub fn set(&self, name: &str, value: SignalValue) -> Result<()> {
        self.signals.insert(name.to_string(), value);
        Ok(())
    }
    
    pub fn get(&self, name: &str) -> Result<SignalValue> {
        self.signals
            .get(name)
            .map(|entry| entry.value().clone())
            .ok_or_else(|| PlcError::SignalNotFound(name.to_string()))
    }
    
    pub fn get_bool(&self, name: &str) -> Result<bool> {
        let value = self.get(name)?;
        value.as_bool()
            .ok_or_else(|| PlcError::TypeMismatch {
                expected: "bool".to_string(),
                actual: value.type_name().to_string(),
            })
    }
    
    pub fn get_int(&self, name: &str) -> Result<i32> {
        let value = self.get(name)?;
        value.as_int()
            .ok_or_else(|| PlcError::TypeMismatch {
                expected: "int".to_string(),
                actual: value.type_name().to_string(),
            })
    }
    
    pub fn get_float(&self, name: &str) -> Result<f64> {
        let value = self.get(name)?;
        value.as_float()
            .ok_or_else(|| PlcError::TypeMismatch {
                expected: "float".to_string(),
                actual: value.type_name().to_string(),
            })
    }
    
    pub fn exists(&self, name: &str) -> bool {
        self.signals.contains_key(name)
    }
    
    pub fn clear(&self) {
        self.signals.clear();
    }
    
    // Return a Vec instead of an iterator to avoid lifetime issues
    pub fn iter(&self) -> Vec<(String, SignalValue)> {
        self.signals.iter().map(|entry| (entry.key().clone(), entry.value().clone())).collect()
    }
}
