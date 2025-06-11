use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

/// Equal comparison block
pub struct EqBlock {
    name: String,
    input1: String,
    input2: String,
    output: String,
}

impl EqBlock {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let input1 = inputs.get("in1")
            .ok_or_else(|| crate::PlcError::ConfigError("EQ requires 'in1' input".to_string()))?
            .clone();
            
        let input2 = inputs.get("in2")
            .ok_or_else(|| crate::PlcError::ConfigError("EQ requires 'in2' input".to_string()))?
            .clone();
            
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("EQ requires 'out' output".to_string()))?
            .clone();
            
        Ok(Self { name, input1, input2, output })
    }
}

impl Block for EqBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let val1 = bus.get(&self.input1)?;
        let val2 = bus.get(&self.input2)?;
        
        let result = match (&val1, &val2) {
            (SignalValue::Bool(a), SignalValue::Bool(b)) => a == b,
            (SignalValue::Int(a), SignalValue::Int(b)) => a == b,
            (SignalValue::Float(a), SignalValue::Float(b)) => (a - b).abs() < f64::EPSILON,
            (SignalValue::String(a), SignalValue::String(b)) => a == b,
            _ => false,
        };
        
        bus.set(&self.output, SignalValue::Bool(result))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "EQ"
    }
}

/// Greater than comparison block
pub struct GtBlock {
    name: String,
    input1: String,
    input2: String,
    output: String,
}

impl GtBlock {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let input1 = inputs.get("in1")
            .ok_or_else(|| crate::PlcError::ConfigError("GT requires 'in1' input".to_string()))?
            .clone();
            
        let input2 = inputs.get("in2")
            .ok_or_else(|| crate::PlcError::ConfigError("GT requires 'in2' input".to_string()))?
            .clone();
            
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("GT requires 'out' output".to_string()))?
            .clone();
            
        Ok(Self { name, input1, input2, output })
    }
}

impl Block for GtBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let val1 = bus.get(&self.input1)?;
        let val2 = bus.get(&self.input2)?;
        
        let result = match (&val1, &val2) {
            (SignalValue::Int(a), SignalValue::Int(b)) => a > b,
            (SignalValue::Float(a), SignalValue::Float(b)) => a > b,
            _ => return Err(crate::PlcError::TypeMismatch {
                expected: "numeric".to_string(),
                actual: "non-numeric".to_string(),
            }),
        };
        
        bus.set(&self.output, SignalValue::Bool(result))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "GT"
    }
}

/// Less than comparison block
pub struct LtBlock {
    name: String,
    input1: String,
    input2: String,
    output: String,
}

impl LtBlock {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let input1 = inputs.get("in1")
            .ok_or_else(|| crate::PlcError::ConfigError("LT requires 'in1' input".to_string()))?
            .clone();
            
        let input2 = inputs.get("in2")
            .ok_or_else(|| crate::PlcError::ConfigError("LT requires 'in2' input".to_string()))?
            .clone();
            
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("LT requires 'out' output".to_string()))?
            .clone();
            
        Ok(Self { name, input1, input2, output })
    }
}

impl Block for LtBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let val1 = bus.get(&self.input1)?;
        let val2 = bus.get(&self.input2)?;
        
        let result = match (&val1, &val2) {
            (SignalValue::Int(a), SignalValue::Int(b)) => a < b,
            (SignalValue::Float(a), SignalValue::Float(b)) => a < b,
            _ => return Err(crate::PlcError::TypeMismatch {
                expected: "numeric".to_string(),
                actual: "non-numeric".to_string(),
            }),
        };
        
        bus.set(&self.output, SignalValue::Bool(result))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "LT"
    }
}
