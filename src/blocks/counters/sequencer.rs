use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

/// Sequencer - Simple incrementing counter with wrap-around
/// Increments on rising edge of trigger, wraps at max value
/// Perfect for rotating equipment, pump alternation, etc.
pub struct Sequencer {
    name: String,
    trigger: String,
    reset: String,
    index_output: String,
    max: i32,
    current_index: i32,
    prev_trigger: bool,
}

impl Sequencer {
    pub fn new(
        name: String,
        inputs: &HashMap<String, String>,
        outputs: &HashMap<String, String>,
        params: &HashMap<String, serde_yaml::Value>
    ) -> Result<Self> {
        let trigger = inputs.get("trigger")
            .ok_or_else(|| crate::PlcError::ConfigError("SEQUENCER requires 'trigger' input".to_string()))?
            .clone();
            
        let reset = inputs.get("reset")
            .ok_or_else(|| crate::PlcError::ConfigError("SEQUENCER requires 'reset' input".to_string()))?
            .clone();
            
        let index_output = outputs.get("index")
            .ok_or_else(|| crate::PlcError::ConfigError("SEQUENCER requires 'index' output".to_string()))?
            .clone();
            
        let max = params.get("max")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| crate::PlcError::ConfigError("SEQUENCER requires 'max' parameter".to_string()))?;
            
        if max <= 0 {
            return Err(crate::PlcError::ConfigError("SEQUENCER 'max' must be positive".to_string()));
        }
            
        Ok(Self {
            name,
            trigger,
            reset,
            index_output,
            max,
            current_index: 0,
            prev_trigger: false,
        })
    }
}

impl Block for Sequencer {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        // Check reset first (highest priority)
        if bus.get_bool(&self.reset)? {
            self.current_index = 0;
            self.prev_trigger = false;
        } else {
            // Check for rising edge on trigger
            let current_trigger = bus.get_bool(&self.trigger)?;
            
            if current_trigger && !self.prev_trigger {
                // Increment and wrap
                self.current_index = (self.current_index + 1) % self.max;
            }
            
            self.prev_trigger = current_trigger;
        }
        
        // Always output current index
        bus.set(&self.index_output, SignalValue::Int(self.current_index))?;
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "SEQUENCER"
    }
}
