use crate::{Result, signal::{SignalBus, SignalValue}};
use crate::blocks::traits::Block;
use std::collections::HashMap;
use std::time::Instant;

/// Timer Pulse - generates a pulse of preset duration on rising edge of input
pub struct TP {
    name: String,
    input: String,
    output: String,
    elapsed_output: Option<String>,
    preset_ms: u64,
    start_time: Option<Instant>,
    elapsed_ms: u64,
    prev_input: bool,
    pulse_active: bool,
}

impl TP {
    pub fn new(
        name: String, 
        inputs: &HashMap<String, String>, 
        outputs: &HashMap<String, String>,
        params: &HashMap<String, serde_yaml::Value>
    ) -> Result<Self> {
        let input = inputs.get("in")
            .ok_or_else(|| crate::PlcError::ConfigError("TP requires 'in' input".to_string()))?
            .clone();
            
        let output = outputs.get("q")
            .ok_or_else(|| crate::PlcError::ConfigError("TP requires 'q' output".to_string()))?
            .clone();
            
        let elapsed_output = outputs.get("et").cloned();
        
        let preset_ms = params.get("preset_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| crate::PlcError::ConfigError("TP requires 'preset_ms' parameter".to_string()))?;
            
        Ok(Self {
            name,
            input,
            output,
            elapsed_output,
            preset_ms,
            start_time: None,
            elapsed_ms: 0,
            prev_input: false,
            pulse_active: false,
        })
    }
}

impl Block for TP {
    fn execute(&mut self, bus: &SignalBus) -> Result<()> {
        let current_input = bus.get_bool(&self.input)?;
        
        // Detect rising edge
        if current_input && !self.prev_input && !self.pulse_active {
            // Start pulse
            self.start_time = Some(Instant::now());
            self.elapsed_ms = 0;
            self.pulse_active = true;
        }
        
        // Update timing if pulse is active
        if self.pulse_active && self.start_time.is_some() {
            self.elapsed_ms = self.start_time.unwrap().elapsed().as_millis() as u64;
            
            // Check if pulse duration exceeded
            if self.elapsed_ms >= self.preset_ms {
                self.pulse_active = false;
                self.start_time = None;
            }
        }
        
        self.prev_input = current_input;
        
        // Set outputs
        bus.set(&self.output, SignalValue::Bool(self.pulse_active))?;
        
        if let Some(et_output) = &self.elapsed_output {
            bus.set(et_output, SignalValue::Int(self.elapsed_ms as i32))?;
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn block_type(&self) -> &str {
        "TP"
    }
}
