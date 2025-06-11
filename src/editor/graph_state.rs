use super::{PlcDataType, PlcValueType, PlcNodeData};
use egui_node_graph::*;
use std::collections::HashMap;
use std::borrow::Cow;

pub type PlcGraph = Graph<PlcNodeData, PlcDataType, PlcValueType>;
pub type PlcEditorState = GraphEditorState<PlcNodeData, PlcDataType, PlcValueType, super::PlcNodeTemplate, PlcGraphState>;

#[derive(Default)]
pub struct PlcGraphState {
    pub active_node: Option<NodeId>,
    pub node_positions: HashMap<NodeId, egui::Pos2>,
    pub simulation_values: HashMap<OutputId, PlcValueType>,
}

impl DataTypeTrait<PlcGraphState> for PlcDataType {
    fn data_type_color(&self, _user_state: &mut PlcGraphState) -> egui::Color32 {
        match self {
            PlcDataType::Bool => egui::Color32::from_rgb(100, 200, 100),
            PlcDataType::Int => egui::Color32::from_rgb(100, 100, 200),
            PlcDataType::Float => egui::Color32::from_rgb(200, 100, 200),
            PlcDataType::String => egui::Color32::from_rgb(200, 200, 100),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            PlcDataType::Bool => Cow::Borrowed("Bool"),
            PlcDataType::Int => Cow::Borrowed("Int"),
            PlcDataType::Float => Cow::Borrowed("Float"),
            PlcDataType::String => Cow::Borrowed("String"),
        }
    }
}

impl NodeDataTrait for PlcNodeData {
    type Response = PlcNodeResponse;
    type UserState = PlcGraphState;
    type DataType = PlcDataType;
    type ValueType = PlcValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: UserResponseTrait,
    {
        let mut responses = Vec::new();
        
        // Property editing UI for different node types
        match self {
            PlcNodeData::TimerOn { preset_ms } => {
                ui.horizontal(|ui| {
                    ui.label("Preset:");
                    let mut preset = *preset_ms as f32 / 1000.0;
                    if ui.add(egui::DragValue::new(&mut preset).suffix(" s").speed(0.1)).changed() {
                        let new_preset = (preset * 1000.0) as u64;
                        responses.push(NodeResponse::User(PlcNodeResponse::SetTimerPreset(new_preset)));
                    }
                });
            }
            PlcNodeData::Counter { preset } => {
                ui.horizontal(|ui| {
                    ui.label("Preset:");
                    let mut preset_val = *preset;
                    if ui.add(egui::DragValue::new(&mut preset_val)).changed() {
                        responses.push(NodeResponse::User(PlcNodeResponse::SetCounterPreset(preset_val)));
                    }
                });
            }
            PlcNodeData::PID { kp, ki, kd } => {
                let mut changed = false;
                let mut new_kp = *kp;
                let mut new_ki = *ki;
                let mut new_kd = *kd;
                
                ui.horizontal(|ui| {
                    ui.label("Kp:");
                    if ui.add(egui::DragValue::new(&mut new_kp).speed(0.01)).changed() {
                        changed = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Ki:");
                    if ui.add(egui::DragValue::new(&mut new_ki).speed(0.01)).changed() {
                        changed = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Kd:");
                    if ui.add(egui::DragValue::new(&mut new_kd).speed(0.01)).changed() {
                        changed = true;
                    }
                });
                
                if changed {
                    responses.push(NodeResponse::User(PlcNodeResponse::SetPIDParams(new_kp, new_ki, new_kd)));
                }
            }
            PlcNodeData::Constant { value } => {
                match value {
                    PlcValueType::Bool(b) => {
                        let mut val = *b;
                        if ui.checkbox(&mut val, "Value").changed() {
                            responses.push(NodeResponse::User(PlcNodeResponse::SetConstantValue(PlcValueType::Bool(val))));
                        }
                    }
                    PlcValueType::Int(i) => {
                        let mut val = *i;
                        if ui.add(egui::DragValue::new(&mut val)).changed() {
                            responses.push(NodeResponse::User(PlcNodeResponse::SetConstantValue(PlcValueType::Int(val))));
                        }
                    }
                    PlcValueType::Float(f) => {
                        let mut val = *f;
                        if ui.add(egui::DragValue::new(&mut val).speed(0.1)).changed() {
                            responses.push(NodeResponse::User(PlcNodeResponse::SetConstantValue(PlcValueType::Float(val))));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        responses
    }
}

#[derive(Debug, Clone)]
pub enum PlcNodeResponse {
    SetTimerPreset(u64),
    SetCounterPreset(i32),
    SetPIDParams(f64, f64, f64),
    SetConstantValue(PlcValueType),
}

impl UserResponseTrait for PlcNodeResponse {}

impl WidgetValueTrait for PlcValueType {
    type Response = PlcNodeResponse;
    type UserState = PlcGraphState;
    type NodeData = PlcNodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        let responses = Vec::new();
        
        ui.label(param_name);
        
        match self {
            PlcValueType::Bool(val) => {
                ui.checkbox(val, "");
            }
            PlcValueType::Int(val) => {
                ui.add(egui::DragValue::new(val));
            }
            PlcValueType::Float(val) => {
                ui.add(egui::DragValue::new(val).speed(0.1));
            }
            PlcValueType::String(val) => {
                ui.text_edit_singleline(val);
            }
            PlcValueType::None => {
                ui.label("None");
            }
        }
        
        responses
    }
}
