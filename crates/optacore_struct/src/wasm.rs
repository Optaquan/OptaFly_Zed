//! WASM bindings for browser-based architecture optimization
//!
//! Enables OptaCore-Struct to run entirely in-browser for:
//! - Privacy-first diagramming (no data leaves device)
//! - Instant interactivity (<100ms layout optimization)
//! - Offline-capable PWA deployment
//! - Edge deployment for OptaEdge assistant

use crate::{NodeType, OptaModel, OptaNode, OptaOptimizer, detect_anti_patterns, parse_c4_dsl};
use burn::backend::ndarray::NdArray;
use wasm_bindgen::prelude::*;

type WasmBackend = NdArray<f32>;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub struct WasmOptaModel {
    inner: OptaModel<WasmBackend>,
}

#[wasm_bindgen]
impl WasmOptaModel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log!("OptaCore-Struct initialized");
        Self {
            inner: OptaModel::new(),
        }
    }

    #[wasm_bindgen(js_name = addNode)]
    pub fn add_node(&mut self, id: String, name: String, node_type: String) -> Result<(), JsValue> {
        let nt = match node_type.as_str() {
            "system" => NodeType::System,
            "container" => NodeType::Container,
            "component" => NodeType::Component,
            "person" => NodeType::Person,
            _ => {
                return Err(JsValue::from_str(&format!(
                    "Invalid node type: {}",
                    node_type
                )));
            }
        };

        self.inner.add_node(OptaNode::new(id, name, nt));
        Ok(())
    }

    #[wasm_bindgen(js_name = addEdge)]
    pub fn add_edge(&mut self, from: String, to: String, label: Option<String>) {
        let mut edge = crate::model::OptaEdge::new(from, to);
        if let Some(l) = label {
            edge = edge.with_label(l);
        }
        self.inner.add_edge(edge);
    }

    #[wasm_bindgen(js_name = buildAdjacencyMatrix)]
    pub fn build_adjacency_matrix(&mut self) {
        self.inner.build_adjacency_matrix();
    }

    #[wasm_bindgen(js_name = optimize)]
    pub fn optimize(&mut self, iterations: usize) -> Result<(), JsValue> {
        let optimizer = OptaOptimizer::<WasmBackend>::new(iterations, 0.1);
        optimizer
            .optimize_layout(&mut self.inner)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = detectAntiPatterns)]
    pub fn detect_anti_patterns(&self) -> Result<String, JsValue> {
        let patterns =
            detect_anti_patterns(&self.inner).map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_json::to_string(&patterns).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        let nodes: Vec<_> = self
            .inner
            .nodes
            .iter()
            .map(|n| {
                serde_json::json!({
                    "id": n.id,
                    "name": n.name,
                    "type": match n.node_type {
                        NodeType::System => "system",
                        NodeType::Container => "container",
                        NodeType::Component => "component",
                        NodeType::Person => "person",
                    },
                    "position": n.get_position(),
                })
            })
            .collect();

        let edges: Vec<_> = self
            .inner
            .edges
            .iter()
            .map(|e| {
                serde_json::json!({
                    "from": e.from,
                    "to": e.to,
                    "label": e.label,
                })
            })
            .collect();

        serde_json::json!({
            "nodes": nodes,
            "edges": edges,
        })
        .to_string()
        .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = nodeCount)]
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    #[wasm_bindgen(js_name = edgeCount)]
    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }
}

#[wasm_bindgen(js_name = parseC4Dsl)]
pub fn parse_c4_dsl_wasm(input: &str) -> Result<WasmOptaModel, JsValue> {
    let model =
        parse_c4_dsl::<WasmBackend>(input).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(WasmOptaModel { inner: model })
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "wasm")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::panic_hook));
        console_log!("OptaCore-Struct WASM module loaded");
    }
}
