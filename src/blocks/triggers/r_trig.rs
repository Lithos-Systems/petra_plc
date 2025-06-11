use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

/// Rising edge trigger - outputs true for one scan when input transitions from false to true
pub struct RTrig {
    name: String,
    input: String,
    output: String,
    prev_state: bool,
}

impl RTrig {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let input = inputs.get("clk")
            .ok_or_else(|| crate::PlcError::ConfigError("R_TRIG requires 'clk' input".to_string()))?
            .clone();
            
        let output = outputs.get("q")
            .ok_or_else(|| crate::PlcError::ConfigError("R_TRIG requires 'q' output".to_string()))?
            .clone();
            
        Ok(Self {
            name,
            input,
            output,
            prev_state: false,
        })
    }
}

impl Block for RTrig {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let current = bus.get_bool(&self.input)?;
        let rising_edge = current && !self.prev_state;
        self.prev_state = current;
        
        bus.set(&self.output, SignalValue::Bool(rising_edge))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "R_TRIG"
    }
}
