use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumCount};

/// Data types that can flow through connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum PlcDataType {
    Bool,
    Int,
    Float,
    String,
}

/// Actual values stored in connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlcValueType {
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
    None,
}

// Add Default implementation for PlcValueType
impl Default for PlcValueType {
    fn default() -> Self {
        PlcValueType::None
    }
}

/// Node data for different PLC block types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlcNodeData {
    // Logic blocks
    And { num_inputs: usize },
    Or { num_inputs: usize },
    Not,
    
    // Comparison
    GreaterThan,
    LessThan,
    Equal,
    
    // Triggers
    RisingEdge,
    FallingEdge,
    SRLatch,
    
    // Timers
    TimerOn { preset_ms: u64 },
    TimerOff { preset_ms: u64 },
    TimerPulse { preset_ms: u64 },
    
    // Counters
    Counter { preset: i32 },
    Sequencer { max: i32 },
    
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    
    // Control
    PID { kp: f64, ki: f64, kd: f64 },
    
    // I/O
    Input { signal_name: String, data_type: PlcDataType },
    Output { signal_name: String },
    Constant { value: PlcValueType },
}

impl PlcNodeData {
    pub fn node_color(&self) -> egui::Color32 {
        match self {
            PlcNodeData::And { .. } | PlcNodeData::Or { .. } | PlcNodeData::Not => 
                egui::Color32::from_rgb(150, 150, 200), // Logic - blue
            PlcNodeData::GreaterThan | PlcNodeData::LessThan | PlcNodeData::Equal => 
                egui::Color32::from_rgb(200, 150, 150), // Comparison - red
            PlcNodeData::RisingEdge | PlcNodeData::FallingEdge | PlcNodeData::SRLatch => 
                egui::Color32::from_rgb(150, 200, 150), // Triggers - green
            PlcNodeData::TimerOn { .. } | PlcNodeData::TimerOff { .. } | PlcNodeData::TimerPulse { .. } => 
                egui::Color32::from_rgb(200, 200, 150), // Timers - yellow
            PlcNodeData::Counter { .. } | PlcNodeData::Sequencer { .. } => 
                egui::Color32::from_rgb(200, 150, 200), // Counters - magenta
            PlcNodeData::Add | PlcNodeData::Subtract | PlcNodeData::Multiply | PlcNodeData::Divide => 
                egui::Color32::from_rgb(150, 200, 200), // Math - cyan
            PlcNodeData::PID { .. } => 
                egui::Color32::from_rgb(200, 180, 150), // Control - orange
            PlcNodeData::Input { .. } => 
                egui::Color32::from_rgb(150, 250, 150), // Input - bright green
            PlcNodeData::Output { .. } => 
                egui::Color32::from_rgb(250, 150, 150), // Output - bright red
            PlcNodeData::Constant { .. } => 
                egui::Color32::from_rgb(180, 180, 180), // Constant - gray
        }
    }
}
