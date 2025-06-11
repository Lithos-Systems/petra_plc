use crate::{Result, signal::SignalBus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Block: Send + Sync {
    fn execute(&mut self, bus: &SignalBus) -> Result<()>;
    fn name(&self) -> &str;
    fn block_type(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub inputs: HashMap<String, String>,
    pub outputs: HashMap<String, String>,
    #[serde(default)]
    pub params: HashMap<String, serde_yaml::Value>,
}
