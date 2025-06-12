use super::{PlcNodeData, PlcDataType, PlcValueType};
use egui_node_graph::*;
use strum_macros::EnumIter;
use std::borrow::Cow;

/// Categories for organizing nodes in the UI
#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum PlcNodeTemplateCategory {
    Logic,
    Comparison,
    Triggers,
    Timers,
    Counters,
    Math,
    Control,
    IO,
}

impl PlcNodeTemplateCategory {
    pub fn name(&self) -> &str {
        match self {
            PlcNodeTemplateCategory::Logic => "Logic",
            PlcNodeTemplateCategory::Comparison => "Comparison",
            PlcNodeTemplateCategory::Triggers => "Triggers",
            PlcNodeTemplateCategory::Timers => "Timers",
            PlcNodeTemplateCategory::Counters => "Counters",
            PlcNodeTemplateCategory::Math => "Math",
            PlcNodeTemplateCategory::Control => "Control",
            PlcNodeTemplateCategory::IO => "I/O",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlcNodeTemplate {
    pub name: String,
    pub category: PlcNodeTemplateCategory,
    pub node_data: PlcNodeData,
    pub inputs: Vec<(&'static str, PlcDataType)>,
    pub outputs: Vec<(&'static str, PlcDataType)>,
}

impl PlcNodeTemplate {
    pub fn all_templates() -> PlcNodeTemplates {
        PlcNodeTemplates(vec![
            // Logic
            Self {
                name: "AND".to_string(),
                category: PlcNodeTemplateCategory::Logic,
                node_data: PlcNodeData::And { num_inputs: 2 },
                inputs: vec![("in1", PlcDataType::Bool), ("in2", PlcDataType::Bool)],
                outputs: vec![("out", PlcDataType::Bool)],
            },
            Self {
                name: "OR".to_string(),
                category: PlcNodeTemplateCategory::Logic,
                node_data: PlcNodeData::Or { num_inputs: 2 },
                inputs: vec![("in1", PlcDataType::Bool), ("in2", PlcDataType::Bool)],
                outputs: vec![("out", PlcDataType::Bool)],
            },
            Self {
                name: "NOT".to_string(),
                category: PlcNodeTemplateCategory::Logic,
                node_data: PlcNodeData::Not,
                inputs: vec![("in", PlcDataType::Bool)],
                outputs: vec![("out", PlcDataType::Bool)],
            },
            
            // Timers
            Self {
                name: "Timer ON".to_string(),
                category: PlcNodeTemplateCategory::Timers,
                node_data: PlcNodeData::TimerOn { preset_ms: 1000 },
                inputs: vec![("in", PlcDataType::Bool)],
                outputs: vec![("q", PlcDataType::Bool), ("et", PlcDataType::Int)],
            },
            
            // I/O
            Self {
                name: "Bool Input".to_string(),
                category: PlcNodeTemplateCategory::IO,
                node_data: PlcNodeData::Input { 
                    signal_name: "input_signal".to_string(),
                    data_type: PlcDataType::Bool 
                },
                inputs: vec![],
                outputs: vec![("value", PlcDataType::Bool)],
            },
            Self {
                name: "Bool Constant".to_string(),
                category: PlcNodeTemplateCategory::IO,
                node_data: PlcNodeData::Constant { value: PlcValueType::Bool(false) },
                inputs: vec![],
                outputs: vec![("value", PlcDataType::Bool)],
            },
            
            // Add more templates as needed...
        ])
    }
}


/// Container type for passing node templates to the graph editor APIs.
#[derive(Clone, Debug)]
pub struct PlcNodeTemplates(pub Vec<PlcNodeTemplate>);

impl From<Vec<PlcNodeTemplate>> for PlcNodeTemplates {
    fn from(v: Vec<PlcNodeTemplate>) -> Self {
        PlcNodeTemplates(v)
    }
}

impl std::ops::Deref for PlcNodeTemplates {
    type Target = [PlcNodeTemplate];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for PlcNodeTemplates {
    type Item = PlcNodeTemplate;
    type IntoIter = std::vec::IntoIter<PlcNodeTemplate>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a PlcNodeTemplates {
    type Item = &'a PlcNodeTemplate;
    type IntoIter = std::slice::Iter<'a, PlcNodeTemplate>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl egui_node_graph::NodeTemplateIter for PlcNodeTemplates {
    type Item = PlcNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0.clone()
    }
}

impl NodeTemplateTrait for PlcNodeTemplate {
    type NodeData = PlcNodeData;
    type DataType = PlcDataType;
    type ValueType = PlcValueType;
    type UserState = super::PlcGraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into_owned()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        self.node_data.clone()
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // Add inputs
        for (name, data_type) in &self.inputs {
            graph.add_input_param(
                node_id, 
                name.to_string(), 
                *data_type,
                PlcValueType::default(),
                InputParamKind::ConnectionOrConstant,
                true
            );
        }
        
        // Add outputs
        for (name, data_type) in &self.outputs {
            graph.add_output_param(node_id, name.to_string(), *data_type);
        }
    }
}
