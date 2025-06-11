use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

pub struct SRLatch {
    name: String,
    set_input: String,
    reset_input: String,
    output: String,
    state: bool,
}

impl SRLatch {
    pub fn new(name: String, inputs: &HashMap<String, String>, outputs: &HashMap<String, String>) -> Result<Self> {
        let set_input = inputs.get("set")
            .ok_or_else(|| crate::PlcError::ConfigError("SR_LATCH requires 'set' input".to_string()))?
            .clone();
            
        let reset_input = inputs.get("reset")
            .ok_or_else(|| crate::PlcError::ConfigError("SR_LATCH requires 'reset' input".to_string()))?
            .clone();
            
        let output = outputs.get("q")
            .ok_or_else(|| crate::PlcError::ConfigError("SR_LATCH requires 'q' output".to_string()))?
            .clone();
            
        Ok(Self {
            name,
            set_input,
            reset_input,
            output,
            state: false,
        })
    }
}

impl Block for SRLatch {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let set = bus.get_bool(&self.set_input)?;
        let reset = bus.get_bool(&self.reset_input)?;
        
        // Reset has priority
        if reset {
            self.state = false;
        } else if set {
            self.state = true;
        }
        
        bus.set(&self.output, SignalValue::Bool(self.state))?;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "SR_LATCH"
    }
}
