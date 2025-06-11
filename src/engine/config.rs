use serde::{Deserialize, Serialize};
use crate::{Result, PlcError, signal::SignalValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub signal_type: String,
    #[serde(default)]
    pub initial: serde_yaml::Value,
}

impl SignalConfig {
    pub fn to_signal_value(&self) -> Result<SignalValue> {
        match self.signal_type.as_str() {
            "bool" => {
                let val = self.initial.as_bool().unwrap_or(false);
                Ok(SignalValue::Bool(val))
            }
            "int" => {
                let val = self.initial.as_i64().unwrap_or(0) as i32;
                Ok(SignalValue::Int(val))
            }
            "float" => {
                let val = self.initial.as_f64().unwrap_or(0.0);
                Ok(SignalValue::Float(val))
            }
            "string" => {
                let val = self.initial.as_str().unwrap_or("").to_string();
                Ok(SignalValue::String(val))
            }
            _ => Err(PlcError::ConfigError(format!(
                "Unknown signal type: {}",
                self.signal_type
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlcConfig {
    #[serde(default)]
    pub signals: Vec<SignalConfig>,
    #[serde(default)]
    pub blocks: Vec<crate::blocks::BlockConfig>,
    #[serde(default)]
    pub scan_time_ms: u64,
}

impl Default for PlcConfig {
    fn default() -> Self {
        Self {
            signals: Vec::new(),
            blocks: Vec::new(),
            scan_time_ms: 100, // Default 100ms scan time
        }
    }
}

impl PlcConfig {
    pub fn from_yaml(yaml_str: &str) -> Result<Self> {
        serde_yaml::from_str(yaml_str)
            .map_err(|e| PlcError::YamlError(e))
    }
    
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Self::from_yaml(&contents)
    }
}
