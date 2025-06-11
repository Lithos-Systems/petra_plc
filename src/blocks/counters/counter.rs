use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;

/// Up/Down Counter with preset value
pub struct Counter {
    name: String,
    count_up: String,
    count_down: String,
    reset: String,
    preset_input: Option<String>,
    output: String,
    done_output: Option<String>,
    preset: i32,
    count: i32,
    prev_up: bool,
    prev_down: bool,
}

impl Counter {
    pub fn new(
        name: String,
        inputs: &HashMap<String, String>,
        outputs: &HashMap<String, String>,
        params: &HashMap<String, serde_yaml::Value>
    ) -> Result<Self> {
        let count_up = inputs.get("cu")
            .ok_or_else(|| crate::PlcError::ConfigError("COUNTER requires 'cu' input".to_string()))?
            .clone();
            
        let count_down = inputs.get("cd")
            .ok_or_else(|| crate::PlcError::ConfigError("COUNTER requires 'cd' input".to_string()))?
            .clone();
            
        let reset = inputs.get("r")
            .ok_or_else(|| crate::PlcError::ConfigError("COUNTER requires 'r' input".to_string()))?
            .clone();
            
        let preset_input = inputs.get("pv").cloned();
        
        let output = outputs.get("cv")
            .ok_or_else(|| crate::PlcError::ConfigError("COUNTER requires 'cv' output".to_string()))?
            .clone();
            
        let done_output = outputs.get("q").cloned();
        
        let preset = params.get("preset")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(0);
            
        Ok(Self {
            name,
            count_up,
            count_down,
            reset,
            preset_input,
            output,
            done_output,
            preset,
            count: 0,
            prev_up: false,
            prev_down: false,
        })
    }
}

impl Block for Counter {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        // Check reset first
        if bus.get_bool(&self.reset)? {
            self.count = 0;
        } else {
            // Get current preset value if input is connected
            if let Some(pv) = &self.preset_input {
                if let Ok(preset_value) = bus.get_int(pv) {
                    self.preset = preset_value;
                }
            }
            
            // Check for count up edge
            let current_up = bus.get_bool(&self.count_up)?;
            if current_up && !self.prev_up {
                self.count += 1;
            }
            self.prev_up = current_up;
            
            // Check for count down edge
            let current_down = bus.get_bool(&self.count_down)?;
            if current_down && !self.prev_down {
                self.count -= 1;
            }
            self.prev_down = current_down;
        }
        
        // Set outputs
        bus.set(&self.output, SignalValue::Int(self.count))?;
        
        if let Some(done) = &self.done_output {
            bus.set(done, SignalValue::Bool(self.count >= self.preset))?;
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "COUNTER"
    }
}
