use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

/// Falling edge trigger - outputs true for one scan when input transitions from true to false
pub struct FTrig {
    name: String,
    input: String,
    output: String,
    prev_state: bool,
}

impl FTrig {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let input = inputs.get("clk")
            .ok_or_else(|| crate::PlcError::ConfigError("F_TRIG requires 'clk' input".to_string()))?
            .clone();
            
        let output = outputs.get("q")
            .ok_or_else(|| crate::PlcError::ConfigError("F_TRIG requires 'q' output".to_string()))?
            .clone();
            
        Ok(Self {
            name,
            input,
            output,
            prev_state: false,
        })
    }
}

impl Block for FTrig {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let current = bus.get_bool(&self.input)?;
        let falling_edge = !current && self.prev_state;
        self.prev_state = current;
        
        bus.set(&self.output, SignalValue::Bool(falling_edge))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "F_TRIG"
    }
}
