use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;
use std::time::Instant;

/// Timer On Delay - output turns on after input has been true for preset time
pub struct TON {
    name: String,
    input: String,
    output: String,
    elapsed_output: Option<String>,
    preset_ms: u64,
    start_time: Option<Instant>,
    elapsed_ms: u64,
    prev_input: bool,
}

impl TON {
    pub fn new(
        name: String, 
        inputs: &HashMap<String, String>, 
        outputs: &HashMap<String, String>,
        params: &HashMap<String, serde_yaml::Value>
    ) -> Result<Self> {
        let input = inputs.get("in")
            .ok_or_else(|| crate::PlcError::ConfigError("TON requires 'in' input".to_string()))?
            .clone();
            
        let output = outputs.get("q")
            .ok_or_else(|| crate::PlcError::ConfigError("TON requires 'q' output".to_string()))?
            .clone();
            
        let elapsed_output = outputs.get("et").cloned();
        
        let preset_ms = params.get("preset_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| crate::PlcError::ConfigError("TON requires 'preset_ms' parameter".to_string()))?;
            
        Ok(Self {
            name,
            input,
            output,
            elapsed_output,
            preset_ms,
            start_time: None,
            elapsed_ms: 0,
            prev_input: false,
        })
    }
}

impl Block for TON {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let current_input = bus.get_bool(&self.input)?;
        
        if current_input && !self.prev_input {
            // Rising edge - start timing
            self.start_time = Some(Instant::now());
            self.elapsed_ms = 0;
        } else if !current_input {
            // Input is false - reset
            self.start_time = None;
            self.elapsed_ms = 0;
        } else if current_input && self.start_time.is_some() {
            // Input remains true - update elapsed time
            self.elapsed_ms = self.start_time.unwrap().elapsed().as_millis() as u64;
        }
        
        self.prev_input = current_input;
        
        // Set outputs
        let done = current_input && self.elapsed_ms >= self.preset_ms;
        bus.set(&self.output, SignalValue::Bool(done))?;
        
        if let Some(et_output) = &self.elapsed_output {
            bus.set(et_output, SignalValue::Int(self.elapsed_ms as i32))?;
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "TON"
    }
}
