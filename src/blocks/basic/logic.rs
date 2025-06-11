use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

pub struct AndBlock {
    name: String,
    inputs: Vec<String>,
    output: String,
}

impl AndBlock {
    pub fn new(name: String, config: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let inputs: Vec<String> = config.iter()
            .filter(|(k, _)| k.starts_with("in"))
            .map(|(_, v)| v.clone())
            .collect();
            
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("AND block requires 'out' output".to_string()))?
            .clone();
            
        Ok(Self { name, inputs, output })
    }
}

impl Block for AndBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let mut result = true;
        
        for input in &self.inputs {
            result = result && bus.get_bool(input)?;
        }
        
        bus.set(&self.output, SignalValue::Bool(result))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "AND"
    }
}

pub struct OrBlock {
    name: String,
    inputs: Vec<String>,
    output: String,
}

impl OrBlock {
    pub fn new(name: String, config: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let inputs: Vec<String> = config.iter()
            .filter(|(k, _)| k.starts_with("in"))
            .map(|(_, v)| v.clone())
            .collect();
            
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("OR block requires 'out' output".to_string()))?
            .clone();
            
        Ok(Self { name, inputs, output })
    }
}

impl Block for OrBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let mut result = false;
        
        for input in &self.inputs {
            result = result || bus.get_bool(input)?;
        }
        
        bus.set(&self.output, SignalValue::Bool(result))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "OR"
    }
}

pub struct NotBlock {
    name: String,
    input: String,
    output: String,
}

impl NotBlock {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let input = inputs.get("in")
            .ok_or_else(|| crate::PlcError::ConfigError("NOT block requires 'in' input".to_string()))?
            .clone();
            
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("NOT block requires 'out' output".to_string()))?
            .clone();
            
        Ok(Self { name, input, output })
    }
}

impl Block for NotBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let value = bus.get_bool(&self.input)?;
        bus.set(&self.output, SignalValue::Bool(!value))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "NOT"
    }
}
