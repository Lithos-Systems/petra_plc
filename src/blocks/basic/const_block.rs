use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

/// Constant value block - outputs a constant value
pub struct ConstBlock {
    name: String,
    output: String,
    value: SignalValue,
}

impl ConstBlock {
    pub fn new(
        name: String,
        outputs: &HashMap<String, String>,
        params: &HashMap<String, serde_yaml::Value>
    ) -> Result<Self> {
        let output = outputs.get("out")
            .ok_or_else(|| crate::PlcError::ConfigError("CONST requires 'out' output".to_string()))?
            .clone();
            
        let value_param = params.get("value")
            .ok_or_else(|| crate::PlcError::ConfigError("CONST requires 'value' parameter".to_string()))?;
            
        // Determine value type from YAML
        let value = if let Some(b) = value_param.as_bool() {
            SignalValue::Bool(b)
        } else if let Some(i) = value_param.as_i64() {
            SignalValue::Int(i as i32)
        } else if let Some(f) = value_param.as_f64() {
            SignalValue::Float(f)
        } else if let Some(s) = value_param.as_str() {
            SignalValue::String(s.to_string())
        } else {
            return Err(crate::PlcError::ConfigError("CONST value must be bool, int, float, or string".to_string()));
        };
            
        Ok(Self { name, output, value })
    }
}

impl Block for ConstBlock {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        bus.set(&self.output, self.value.clone())?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "CONST"
    }
}
