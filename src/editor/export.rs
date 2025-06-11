use super::{PlcNodeData, PlcDataType};
use crate::blocks::BlockConfig;
use egui_node_graph::{NodeId, OutputId};
use std::collections::HashMap;

pub type PlcGraph = egui_node_graph::Graph<PlcNodeData, PlcDataType, super::PlcValueType>;

pub struct YamlExporter {
    graph: PlcGraph,
    signal_counter: usize,
}

impl YamlExporter {
    pub fn new(graph: PlcGraph) -> Self {
        Self {
            graph,
            signal_counter: 0,
        }
    }
    
    fn generate_signal_name(&mut self, prefix: &str) -> String {
        self.signal_counter += 1;
        format!("{}_{}", prefix, self.signal_counter)
    }
    
    pub fn export_to_config(&mut self) -> crate::engine::PlcConfig {
        let mut signals = Vec::new();
        let mut blocks = Vec::new();
        let mut signal_map: HashMap<OutputId, String> = HashMap::new();

        // First pass: create signals for all connections
        let connections: Vec<_> = self
            .graph
            .connections
            .iter()
            .map(|(input, output)| (*input, *output))
            .collect();

        for (_input_id, output_id) in connections {
            let signal_name = self.generate_signal_name("signal");
            signal_map.insert(output_id, signal_name.clone());

            // Determine signal type from output
            let output = self.graph.get_output(output_id);
            let signal_type = match output.typ {
                PlcDataType::Bool => "bool",
                PlcDataType::Int => "int",
                PlcDataType::Float => "float",
                PlcDataType::String => "string",
            };
            
            signals.push(crate::engine::SignalConfig {
                name: signal_name,
                signal_type: signal_type.to_string(),
                initial: serde_yaml::Value::Null,
            });
        }
        
        // Add signals for inputs and outputs
        for (_node_id, node) in &self.graph.nodes {
            match &node.user_data {
                PlcNodeData::Input { signal_name, data_type } => {
                    let signal_type = match data_type {
                        PlcDataType::Bool => "bool",
                        PlcDataType::Int => "int", 
                        PlcDataType::Float => "float",
                        PlcDataType::String => "string",
                    };
                    
                    // Check if signal already exists
                    if !signals.iter().any(|s| s.name == *signal_name) {
                        signals.push(crate::engine::SignalConfig {
                            name: signal_name.clone(),
                            signal_type: signal_type.to_string(),
                            initial: serde_yaml::Value::Null,
                        });
                    }
                }
                PlcNodeData::Output { signal_name: _ } => {
                    // Output nodes need their input connection mapped
                    // This will be handled in the block creation
                }
                _ => {}
            }
        }
        
        // Second pass: create blocks
        for (node_id, node) in &self.graph.nodes {
            if let Some(block_config) = self.node_to_block_config(node_id, &node, &signal_map) {
                blocks.push(block_config);
            }
        }
        
        crate::engine::PlcConfig {
            signals,
            blocks,
            scan_time_ms: 100, // Default scan time
        }
    }
    
    fn node_to_block_config(
        &self,
        _node_id: NodeId,
        node: &egui_node_graph::Node<PlcNodeData>,
        signal_map: &HashMap<OutputId, String>
    ) -> Option<BlockConfig> {
        let mut inputs = HashMap::new();
        let mut outputs = HashMap::new();
        let mut params = HashMap::new();
        
        // Map inputs
        for (param_name, input_id) in &node.inputs {
            if let Some(output_id) = self.graph.connections.get(*input_id) {
                if let Some(signal_name) = signal_map.get(output_id) {
                    inputs.insert(param_name.clone(), signal_name.clone());
                }
            }
        }
        
        // Map outputs
        for (param_name, output_id) in &node.outputs {
            if let Some(signal_name) = signal_map.get(output_id) {
                outputs.insert(param_name.clone(), signal_name.clone());
            }
        }
        
        // Convert node data to block type and parameters
        let (block_type, extra_params) = match &node.user_data {
            PlcNodeData::And { num_inputs: _ } => ("AND".to_string(), HashMap::new()),
            PlcNodeData::Or { num_inputs: _ } => ("OR".to_string(), HashMap::new()),
            PlcNodeData::Not => ("NOT".to_string(), HashMap::new()),
            PlcNodeData::GreaterThan => ("GT".to_string(), HashMap::new()),
            PlcNodeData::LessThan => ("LT".to_string(), HashMap::new()),
            PlcNodeData::Equal => ("EQ".to_string(), HashMap::new()),
            PlcNodeData::RisingEdge => ("R_TRIG".to_string(), HashMap::new()),
            PlcNodeData::FallingEdge => ("F_TRIG".to_string(), HashMap::new()),
            PlcNodeData::SRLatch => ("SR_LATCH".to_string(), HashMap::new()),
            PlcNodeData::TimerOn { preset_ms } => {
                let mut p = HashMap::new();
                p.insert("preset_ms".to_string(), serde_yaml::Value::from(*preset_ms));
                ("TON".to_string(), p)
            }
            PlcNodeData::TimerOff { preset_ms } => {
                let mut p = HashMap::new();
                p.insert("preset_ms".to_string(), serde_yaml::Value::from(*preset_ms));
                ("TOF".to_string(), p)
            }
            PlcNodeData::TimerPulse { preset_ms } => {
                let mut p = HashMap::new();
                p.insert("preset_ms".to_string(), serde_yaml::Value::from(*preset_ms));
                ("TP".to_string(), p)
            }
            PlcNodeData::Counter { preset } => {
                let mut p = HashMap::new();
                p.insert("preset".to_string(), serde_yaml::Value::from(*preset));
                ("COUNTER".to_string(), p)
            }
            PlcNodeData::Sequencer { max } => {
                let mut p = HashMap::new();
                p.insert("max".to_string(), serde_yaml::Value::from(*max));
                ("SEQUENCER".to_string(), p)
            }
            PlcNodeData::Add => ("ADD".to_string(), HashMap::new()),
            PlcNodeData::Subtract => ("SUB".to_string(), HashMap::new()),
            PlcNodeData::Multiply => ("MUL".to_string(), HashMap::new()),
            PlcNodeData::Divide => ("DIV".to_string(), HashMap::new()),
            PlcNodeData::PID { kp, ki, kd } => {
                let mut p = HashMap::new();
                p.insert("kp".to_string(), serde_yaml::Value::from(*kp));
                p.insert("ki".to_string(), serde_yaml::Value::from(*ki));
                p.insert("kd".to_string(), serde_yaml::Value::from(*kd));
                ("PID".to_string(), p)
            }
            PlcNodeData::Input { signal_name: _, .. } => {
                // Input nodes don't generate blocks, they just reference signals
                return None;
            }
            PlcNodeData::Output { signal_name: _ } => {
                // For output nodes, we need to find what's connected to their input
                // and create an assignment or pass-through
                return None; // For now, skip output nodes
            }
            PlcNodeData::Constant { value } => {
                let mut p = HashMap::new();
                let val = match value {
                    super::PlcValueType::Bool(b) => serde_yaml::Value::from(*b),
                    super::PlcValueType::Int(i) => serde_yaml::Value::from(*i),
                    super::PlcValueType::Float(f) => serde_yaml::Value::from(*f),
                    super::PlcValueType::String(s) => serde_yaml::Value::from(s.clone()),
                    super::PlcValueType::None => serde_yaml::Value::Null,
                };
                p.insert("value".to_string(), val);
                ("CONST".to_string(), p)
            }
        };
        
        params.extend(extra_params);
        
        Some(BlockConfig {
            name: node.label.clone(),
            block_type,
            inputs,
            outputs,
            params,
        })
    }
}

// End of YamlExporter implementation
