use super::{PlcGraphState, PlcNodeTemplate, PlcEditorState, PlcNodeData, PlcNodeResponse};
use super::templates::PlcNodeTemplateCategory;
use egui_node_graph::*;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub struct PlcEditorApp {
    state: PlcEditorState,
    user_state: PlcGraphState,
    
    // UI state
    show_node_finder: bool,
    node_finder_search: String,
    selected_category: Option<PlcNodeTemplateCategory>,
    
    // File handling
    current_file: Option<PathBuf>,
    modified: bool,
}

impl PlcEditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut state = PlcEditorState::default();
        let user_state = PlcGraphState::default();
        
        // Create a simple example graph
        let graph = &mut state.graph;
        
        // Add some example nodes
        let input_node = graph.add_node(
            "Input".to_string(),
            PlcNodeData::Input { 
                signal_name: "start_button".to_string(),
                data_type: super::PlcDataType::Bool 
            },
            |graph, node_id| {
                graph.add_output_param(node_id, "value".to_string(), super::PlcDataType::Bool);
            },
        );
        
        let and_node = graph.add_node(
            "AND".to_string(),
            PlcNodeData::And { num_inputs: 2 },
            |graph, node_id| {
                graph.add_input_param(
                    node_id, 
                    "in1".to_string(), 
                    super::PlcDataType::Bool,
                    super::PlcValueType::default(),
                    InputParamKind::ConnectionOrConstant,
                    true
                );
                graph.add_input_param(
                    node_id, 
                    "in2".to_string(), 
                    super::PlcDataType::Bool,
                    super::PlcValueType::default(),
                    InputParamKind::ConnectionOrConstant,
                    true
                );
                graph.add_output_param(node_id, "out".to_string(), super::PlcDataType::Bool);
            },
        );
        
        // Position nodes
        state.node_positions.insert(input_node, egui::pos2(100.0, 100.0));
        state.node_positions.insert(and_node, egui::pos2(300.0, 100.0));
        
        Self {
            state,
            user_state,
            show_node_finder: false,
            node_finder_search: String::new(),
            selected_category: None,
            current_file: None,
            modified: false,
        }
    }
    
    pub fn export_to_yaml(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut exporter = super::export::YamlExporter::new(self.state.graph.clone());
        let config = exporter.export_to_config();
        Ok(serde_yaml::to_string(&config)?)
    }
}

impl eframe::App for PlcEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.state = PlcEditorState::default();
                        self.current_file = None;
                        self.modified = false;
                        ui.close_menu();
                    }
                    
                    if ui.button("Export to YAML").clicked() {
                        match self.export_to_yaml() {
                            Ok(yaml) => {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("YAML", &["yaml", "yml"])
                                    .save_file()
                                {
                                    match std::fs::write(&path, yaml) {
                                        Ok(_) => println!("Exported to {:?}", path),
                                        Err(e) => eprintln!("Failed to write file: {}", e),
                                    }
                                }
                            }
                            Err(e) => eprintln!("Failed to export: {}", e),
                        }
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("Add Node...").clicked() {
                        self.show_node_finder = true;
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Reset View").clicked() {
                        self.state.pan_zoom = egui_node_graph::PanZoom::default();
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Left panel with node palette
        egui::SidePanel::left("node_palette")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Node Palette");
                ui.separator();
                
                // Category filters
                ui.label("Categories:");
                for category in PlcNodeTemplateCategory::iter() {
                    let selected = self.selected_category == Some(category);
                    if ui.selectable_label(selected, category.name()).clicked() {
                        self.selected_category = if selected { None } else { Some(category) };
                    }
                }
                
                ui.separator();
                
                // Node list
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let templates = PlcNodeTemplate::all_templates();
                    for template in templates.iter() {
                        if let Some(cat) = self.selected_category {
                            if template.category != cat {
                                continue;
                            }
                        }
                        
                        let response = ui.button(&template.name);
                        
                        // Simple click to add
                        if response.clicked() {
                            let graph = &mut self.state.graph;
                            let node_id = graph.add_node(
                                template.name.clone(),
                                template.user_data(&mut self.user_state),
                                |graph, node_id| {
                                    template.build_node(graph, &mut self.user_state, node_id);
                                },
                            );
                            
                            // Position at center of viewport
                            let center = ctx.available_rect().center();
                            self.state.node_positions.insert(node_id, center);
                            self.modified = true;
                        }
                    }
                });
            });
        
        // Right panel with properties
        egui::SidePanel::right("properties_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Properties");
                ui.separator();
                
                if let Some(node_id) = self.user_state.active_node {
                    if let Some(node) = self.state.graph.nodes.get(node_id) {
                        ui.label(format!("Node: {}", node.label));
                        ui.separator();
                        
                        // Show node-specific properties
                        match &node.user_data {
                            PlcNodeData::Input { signal_name, .. } => {
                                ui.label(format!("Signal: {}", signal_name));
                            }
                            PlcNodeData::TimerOn { preset_ms } => {
                                ui.label(format!("Preset: {} ms", preset_ms));
                            }
                            PlcNodeData::PID { kp, ki, kd } => {
                                ui.label(format!("Kp: {}", kp));
                                ui.label(format!("Ki: {}", ki));
                                ui.label(format!("Kd: {}", kd));
                            }
                            _ => {
                                ui.label("No properties");
                            }
                        }
                    }
                } else {
                    ui.label("Select a node to view properties");
                }
            });
        
        // Bottom panel with status
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Nodes: {} | Connections: {}",
                    self.state.graph.nodes.len(),
                    self.state.graph.connections.len()
                ));
                
                if self.modified {
                    ui.label("(modified)");
                }
                
                if let Some(path) = &self.current_file {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(path.display().to_string());
                    });
                }
            });
        });
        
        // Central panel with node graph
        egui::CentralPanel::default().show(ctx, |ui| {
            let graph_response = self.state.draw_graph_editor(
                ui,
                egui_node_graph::AllNodeTemplates(PlcNodeTemplate::all_templates()),
                &mut self.user_state,
            );
            
            // Handle graph responses
            for response in graph_response.node_responses {
                match response {
                    NodeResponse::User(user_response) => {
                        match user_response {
                            PlcNodeResponse::SetTimerPreset(_preset) => {
                                self.modified = true;
                            }
                            PlcNodeResponse::SetCounterPreset(_preset) => {
                                self.modified = true;
                            }
                            PlcNodeResponse::SetPIDParams(_kp, _ki, _kd) => {
                                self.modified = true;
                            }
                            PlcNodeResponse::SetConstantValue(_value) => {
                                self.modified = true;
                            }
                        }
                    }
                    NodeResponse::ConnectEventEnded { .. } => {
                        self.modified = true;
                    }
                    NodeResponse::DisconnectEvent { .. } => {
                        self.modified = true;
                    }
                    NodeResponse::SelectNode(node_id) => {
                        self.user_state.active_node = Some(node_id);
                    }
                    _ => {}
                }
            }
                        
            // Handle node finder
            if self.show_node_finder {
                egui::Window::new("Add Node")
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.text_edit_singleline(&mut self.node_finder_search);
                        
                        egui::ScrollArea::vertical()
                            .max_height(300.0)
                            .show(ui, |ui| {
                                for template in PlcNodeTemplate::all_templates() {
                                    if self.node_finder_search.is_empty() || 
                                       template.name.to_lowercase().contains(&self.node_finder_search.to_lowercase()) {
                                        if ui.button(&template.name).clicked() {
                                            let graph = &mut self.state.graph;
                                            let node_id = graph.add_node(
                                                template.name.clone(),
                                                template.user_data(&mut self.user_state),
                                                |graph, node_id| {
                                                    template.build_node(graph, &mut self.user_state, node_id);
                                                },
                                            );
                                            
                                            // Position at center of viewport
                                            let center = ctx.available_rect().center();
                                            self.state.node_positions.insert(node_id, center);
                                            self.modified = true;
                                            self.show_node_finder = false;
                                            self.node_finder_search.clear();
                                        }
                                    }
                                }
                            });
                        
                        if ui.button("Cancel").clicked() {
                            self.show_node_finder = false;
                            self.node_finder_search.clear();
                        }
                    });
            }
        });
    }
}
