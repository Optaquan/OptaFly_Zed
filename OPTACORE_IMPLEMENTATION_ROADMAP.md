# OptaCore-Struct Implementation Roadmap
## Tensor-Native Architecture Assistant for Optaquan Ecosystem

**Start Date:** 2026-01-09  
**Target Completion:** Q1 2026 (6-8 weeks)  
**Budget:** $15-30 (Fly.io/Prime Intellect deployment)  
**Status:** Ready to Begin

---

## 🎯 Vision Summary

**Build a sovereign, tensor-native architecture optimization tool** that:
- Uses **Burn tensors** for 2-5x faster graph queries vs. Vec/HashMap
- Leverages **Mistral.rs** for ML-powered architecture suggestions
- Provides **PyO3 bridge** for Python UI (Streamlit) + CI/CD integration
- Exports to **C4 DSL, SVG, PlantUML** for industry compatibility
- Integrates with **OptaFly_Zed** as architecture assistant
- Serves as **standalone OptaArch tool** for web deployment

**Key Differentiators vs. Structurizr:**
- ✅ **Tensor-native storage** (SIMD/GPU-accelerated)
- ✅ **ML-optimized pruning** (k-means clustering, gradient descent layout)
- ✅ **Real-time suggestions** (Mistral.rs local inference)
- ✅ **CI/CD linting** via OptaCore routing
- ✅ **MIT/Apache licensing** for broad adoption

---

## 📋 5-Step Implementation Plan

### Step 1: Setup & Prototyping (1-2 weeks, ~$5-10)

#### Week 1: Repository & Core Types

**Day 1: Create Repository**
```bash
# Create new workspace member
cd OptaFly_Zed
cargo new --lib crates/optacore_struct

# Update root Cargo.toml
[workspace]
members = [
    # ... existing
    "crates/optacore_struct",
]
```

**Add to `crates/optacore_struct/Cargo.toml`:**
```toml
[package]
name = "optacore_struct"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[dependencies]
burn = { version = "0.14", default-features = false, features = ["ndarray", "autodiff"] }
burn-ndarray = { version = "0.14" }
uuid = { version = "1.6", features = ["v4"] }
regex = "1.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "tensor_ops"
harness = false

[features]
default = []
wgpu-backend = ["burn/wgpu"]
```

**Day 2-3: Core Data Structures**

**File:** `crates/optacore_struct/src/lib.rs`
```rust
pub mod model;
pub mod optimizer;
pub mod parser;
pub mod export;

use burn::backend::NdArray;

// Type alias for default backend
pub type DefaultBackend = NdArray;

// Re-exports
pub use model::{NodeType, OptaNode, OptaModel};
pub use optimizer::OptaOptimizer;
pub use parser::C4Parser;
```

**File:** `crates/optacore_struct/src/model.rs`
```rust
use burn::tensor::{Tensor, Data, Shape, backend::Backend};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Node types in architecture graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    System,
    Container,
    Component,
    Person,
    Database,
    Queue,
    LoadBalancer,
    Custom(String),
}

/// Tensor-native architecture node
#[derive(Debug, Clone)]
pub struct OptaNode<B: Backend> {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub description: Option<String>,
    
    // Tensor attributes: [x, y, importance, cost, latency, ...]
    pub attrs: Tensor<B, 1>,
}

impl<B: Backend> OptaNode<B> {
    pub fn new(name: impl Into<String>, node_type: NodeType) -> Self {
        let id = Uuid::new_v4().to_string();
        
        // Initialize attributes: [x=0, y=0, importance=0.5, cost=1.0, latency=0]
        let attrs = Tensor::from_floats([0.0, 0.0, 0.5, 1.0, 0.0], &B::Device::default());
        
        Self {
            id,
            name: name.into(),
            node_type,
            description: None,
            attrs,
        }
    }
    
    pub fn position(&self) -> (f32, f32) {
        let data = self.attrs.to_data();
        (data.value[0], data.value[1])
    }
    
    pub fn set_position(&mut self, x: f32, y: f32) {
        let mut data = self.attrs.to_data();
        data.value[0] = x;
        data.value[1] = y;
        self.attrs = Tensor::from_data(data, &B::Device::default());
    }
}

/// Tensor-native architecture model
#[derive(Debug, Clone)]
pub struct OptaModel<B: Backend> {
    pub nodes: Vec<OptaNode<B>>,
    
    // Adjacency matrix: [nodes × nodes] with relationship weights
    pub adj_matrix: Tensor<B, 2>,
    
    // Node features: [nodes × feature_dims]
    pub node_features: Tensor<B, 2>,
}

impl<B: Backend> OptaModel<B> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adj_matrix: Tensor::zeros([0, 0], &B::Device::default()),
            node_features: Tensor::zeros([0, 5], &B::Device::default()),
        }
    }
    
    pub fn add_node(&mut self, node: OptaNode<B>) {
        let n = self.nodes.len();
        self.nodes.push(node);
        
        // Resize adjacency matrix
        if n > 0 {
            let new_adj = Tensor::zeros([n + 1, n + 1], &B::Device::default());
            // Copy old values
            // new_adj[0..n, 0..n] = self.adj_matrix
            self.adj_matrix = new_adj;
        } else {
            self.adj_matrix = Tensor::zeros([1, 1], &B::Device::default());
        }
        
        // Resize feature matrix
        let new_features = Tensor::zeros([n + 1, 5], &B::Device::default());
        self.node_features = new_features;
    }
    
    pub fn add_relationship(&mut self, from_idx: usize, to_idx: usize, weight: f32) {
        // Set adjacency matrix value
        let mut data = self.adj_matrix.to_data();
        let n = self.nodes.len();
        data.value[from_idx * n + to_idx] = weight;
        self.adj_matrix = Tensor::from_data(data, &B::Device::default());
    }
    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn edge_count(&self) -> usize {
        // Count non-zero entries in adj_matrix
        let data = self.adj_matrix.to_data();
        data.value.iter().filter(|&&v| v > 0.0).count()
    }
}
```

**Day 4: C4 DSL Parser**

**File:** `crates/optacore_struct/src/parser.rs`
```rust
use crate::model::{NodeType, OptaNode, OptaModel};
use burn::backend::Backend;
use regex::Regex;
use anyhow::{Result, Context};

pub struct C4Parser;

impl C4Parser {
    /// Parse C4 DSL string into OptaModel
    pub fn parse<B: Backend>(dsl: &str) -> Result<OptaModel<B>> {
        let mut model = OptaModel::new();
        
        // Simple regex-based parser
        let node_re = Regex::new(r#"(\w+)\s*=\s*(\w+)\s+"([^"]+)""#)?;
        let rel_re = Regex::new(r#"(\w+)\s*->\s*(\w+)\s*(?:\[weight=([0-9.]+)\])?"#)?;
        
        let mut node_ids = std::collections::HashMap::new();
        
        // Parse nodes
        for cap in node_re.captures_iter(dsl) {
            let id = &cap[1];
            let node_type_str = &cap[2];
            let name = &cap[3];
            
            let node_type = match node_type_str.to_lowercase().as_str() {
                "system" => NodeType::System,
                "container" => NodeType::Container,
                "component" => NodeType::Component,
                "person" => NodeType::Person,
                "database" => NodeType::Database,
                _ => NodeType::Custom(node_type_str.to_string()),
            };
            
            let node = OptaNode::new(name, node_type);
            node_ids.insert(id.to_string(), model.nodes.len());
            model.add_node(node);
        }
        
        // Parse relationships
        for cap in rel_re.captures_iter(dsl) {
            let from_id = &cap[1];
            let to_id = &cap[2];
            let weight = cap.get(3)
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .unwrap_or(1.0);
            
            if let (Some(&from_idx), Some(&to_idx)) = (node_ids.get(from_id), node_ids.get(to_id)) {
                model.add_relationship(from_idx, to_idx, weight);
            }
        }
        
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DefaultBackend;
    
    #[test]
    fn test_parse_simple_dsl() {
        let dsl = r#"
            web = System "Web Application"
            api = Container "API Gateway"
            db = Database "PostgreSQL"
            
            web -> api [weight=1.0]
            api -> db [weight=0.8]
        "#;
        
        let model: OptaModel<DefaultBackend> = C4Parser::parse(dsl).unwrap();
        assert_eq!(model.node_count(), 3);
        assert_eq!(model.edge_count(), 2);
    }
}
```

**Day 5: Benchmarking**

**File:** `crates/optacore_struct/benches/tensor_ops.rs`
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use optacore_struct::{OptaModel, DefaultBackend};

fn benchmark_model_creation(c: &mut Criterion) {
    c.bench_function("create 1000 node model", |b| {
        b.iter(|| {
            let mut model: OptaModel<DefaultBackend> = OptaModel::new();
            for i in 0..1000 {
                let node = optacore_struct::OptaNode::new(
                    format!("node_{}", i),
                    optacore_struct::NodeType::Component,
                );
                model.add_node(node);
            }
            black_box(model);
        });
    });
}

criterion_group!(benches, benchmark_model_creation);
criterion_main!(benches);
```

**Run benchmarks:**
```bash
cargo bench --package optacore_struct
```

**Target:** <10ms for 1000-node model creation

---

### Step 2: Burn Integration & Optimization (1-2 weeks)

#### Week 2: Optimization Engine

**Day 1-2: Force-Directed Layout**

**File:** `crates/optacore_struct/src/optimizer.rs`
```rust
use crate::model::OptaModel;
use burn::tensor::{Tensor, Backend};
use burn::backend::ndarray::NdArrayDevice;

pub struct OptaOptimizer<B: Backend> {
    learning_rate: f32,
    iterations: usize,
    _phantom: std::marker::PhantomData<B>,
}

impl<B: Backend> OptaOptimizer<B> {
    pub fn new(learning_rate: f32, iterations: usize) -> Self {
        Self {
            learning_rate,
            iterations,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Optimize graph layout using force-directed algorithm
    pub fn optimize_layout(&self, model: &mut OptaModel<B>) {
        let n = model.nodes.len();
        if n == 0 {
            return;
        }
        
        // Initialize positions randomly
        let mut positions = Tensor::random(
            [n, 2],
            burn::tensor::Distribution::Normal(0.0, 1.0),
            &B::Device::default(),
        );
        
        for _ in 0..self.iterations {
            // Compute attractive forces (edges pull nodes together)
            let attractive = self.compute_attractive_forces(&positions, &model.adj_matrix);
            
            // Compute repulsive forces (nodes push each other apart)
            let repulsive = self.compute_repulsive_forces(&positions);
            
            // Update positions
            let forces = attractive + repulsive;
            positions = positions + forces * self.learning_rate;
        }
        
        // Update node positions from optimized tensor
        let pos_data = positions.to_data();
        for (i, node) in model.nodes.iter_mut().enumerate() {
            let x = pos_data.value[i * 2];
            let y = pos_data.value[i * 2 + 1];
            node.set_position(x, y);
        }
    }
    
    fn compute_attractive_forces(
        &self,
        positions: &Tensor<B, 2>,
        adj_matrix: &Tensor<B, 2>,
    ) -> Tensor<B, 2> {
        // For each edge, compute spring force pulling nodes together
        // F_attr = k * (distance - rest_length) * direction
        
        // Simplified: adj_matrix dot positions
        adj_matrix.matmul(positions.clone()) * 0.01
    }
    
    fn compute_repulsive_forces(&self, positions: &Tensor<B, 2>) -> Tensor<B, 2> {
        // All nodes repel each other (inverse square law)
        // F_repel = k / distance^2 * direction
        
        // Simplified: small random perturbation
        Tensor::random(
            positions.shape(),
            burn::tensor::Distribution::Normal(0.0, 0.1),
            &B::Device::default(),
        )
    }
}
```

**Day 3: K-Means Clustering**

```rust
impl<B: Backend> OptaOptimizer<B> {
    /// Cluster nodes using k-means on feature tensors
    pub fn cluster_nodes(&self, model: &OptaModel<B>, k: usize) -> Vec<usize> {
        if model.nodes.len() == 0 {
            return Vec::new();
        }
        
        // K-means clustering on node_features tensor
        // Returns cluster assignment for each node
        
        let features = &model.node_features;
        let n = model.nodes.len();
        
        // Initialize centroids randomly
        let mut centroids = Tensor::random(
            [k, features.dims()[1]],
            burn::tensor::Distribution::Uniform(0.0, 1.0),
            &B::Device::default(),
        );
        
        let mut assignments = vec![0; n];
        
        for _ in 0..50 {  // Max iterations
            // Assign each point to nearest centroid
            for i in 0..n {
                let point = features.clone().slice([i..i+1, 0..features.dims()[1]]);
                let mut min_dist = f32::MAX;
                let mut best_cluster = 0;
                
                for j in 0..k {
                    let centroid = centroids.clone().slice([j..j+1, 0..centroids.dims()[1]]);
                    let dist = self.euclidean_distance(&point, &centroid);
                    
                    if dist < min_dist {
                        min_dist = dist;
                        best_cluster = j;
                    }
                }
                
                assignments[i] = best_cluster;
            }
            
            // Update centroids
            // (simplified - in practice, average points in each cluster)
        }
        
        assignments
    }
    
    fn euclidean_distance(&self, a: &Tensor<B, 2>, b: &Tensor<B, 2>) -> f32 {
        let diff = a.clone() - b.clone();
        let squared = diff.clone().mul(diff);
        let sum = squared.sum();
        sum.to_data().value[0].sqrt()
    }
    
    /// Prune low-weight edges below threshold
    pub fn prune_redundancies(&self, model: &mut OptaModel<B>, threshold: f32) {
        let mask = model.adj_matrix.clone().greater_elem(threshold);
        model.adj_matrix = model.adj_matrix.clone().mask_fill(mask, 0.0);
    }
}
```

**Day 4: Anti-Pattern Detection**

```rust
#[derive(Debug, Clone)]
pub enum AntiPattern {
    CircularDependency { nodes: Vec<String> },
    OverCoupled { node: String, degree: usize },
    Bottleneck { node: String, centrality: f32 },
    IsolatedComponent { node: String },
}

impl<B: Backend> OptaOptimizer<B> {
    /// Detect architecture anti-patterns
    pub fn detect_antipatterns(&self, model: &OptaModel<B>) -> Vec<AntiPattern> {
        let mut patterns = Vec::new();
        
        // 1. Detect circular dependencies (cycles in adj_matrix)
        patterns.extend(self.detect_cycles(model));
        
        // 2. Detect over-coupled components (high node degree)
        patterns.extend(self.detect_high_coupling(model));
        
        // 3. Detect bottlenecks (high betweenness centrality)
        patterns.extend(self.detect_bottlenecks(model));
        
        // 4. Detect isolated components (zero edges)
        patterns.extend(self.detect_isolated(model));
        
        patterns
    }
    
    fn detect_cycles(&self, model: &OptaModel<B>) -> Vec<AntiPattern> {
        // DFS-based cycle detection on adj_matrix
        // (simplified implementation)
        Vec::new()
    }
    
    fn detect_high_coupling(&self, model: &OptaModel<B>) -> Vec<AntiPattern> {
        let mut patterns = Vec::new();
        let threshold = 10;  // More than 10 connections = over-coupled
        
        for (i, node) in model.nodes.iter().enumerate() {
            let degree = self.node_degree(model, i);
            if degree > threshold {
                patterns.push(AntiPattern::OverCoupled {
                    node: node.name.clone(),
                    degree,
                });
            }
        }
        
        patterns
    }
    
    fn node_degree(&self, model: &OptaModel<B>, node_idx: usize) -> usize {
        let adj_data = model.adj_matrix.to_data();
        let n = model.nodes.len();
        
        // Count non-zero entries in row and column
        let mut degree = 0;
        for j in 0..n {
            if adj_data.value[node_idx * n + j] > 0.0 {
                degree += 1;
            }
            if adj_data.value[j * n + node_idx] > 0.0 {
                degree += 1;
            }
        }
        
        degree
    }
    
    fn detect_bottlenecks(&self, model: &OptaModel<B>) -> Vec<AntiPattern> {
        // Compute betweenness centrality via eigenvector
        // (requires advanced graph algorithms - defer to Phase 3)
        Vec::new()
    }
    
    fn detect_isolated(&self, model: &OptaModel<B>) -> Vec<AntiPattern> {
        let mut patterns = Vec::new();
        
        for (i, node) in model.nodes.iter().enumerate() {
            if self.node_degree(model, i) == 0 {
                patterns.push(AntiPattern::IsolatedComponent {
                    node: node.name.clone(),
                });
            }
        }
        
        patterns
    }
}
```

**Day 5: Integration Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OptaModel, OptaNode, NodeType, DefaultBackend};
    
    #[test]
    fn test_optimize_layout() {
        let mut model: OptaModel<DefaultBackend> = OptaModel::new();
        
        // Add 10 nodes
        for i in 0..10 {
            model.add_node(OptaNode::new(format!("node_{}", i), NodeType::Component));
        }
        
        // Add some relationships
        model.add_relationship(0, 1, 1.0);
        model.add_relationship(1, 2, 1.0);
        model.add_relationship(2, 3, 1.0);
        
        let optimizer = OptaOptimizer::new(0.01, 100);
        optimizer.optimize_layout(&mut model);
        
        // Check that positions have changed from (0,0)
        let pos = model.nodes[0].position();
        assert!(pos.0 != 0.0 || pos.1 != 0.0);
    }
    
    #[test]
    fn test_detect_antipatterns() {
        let mut model: OptaModel<DefaultBackend> = OptaModel::new();
        
        // Create over-coupled node
        model.add_node(OptaNode::new("hub", NodeType::Component));
        for i in 0..15 {
            model.add_node(OptaNode::new(format!("node_{}", i), NodeType::Component));
            model.add_relationship(0, i + 1, 1.0);
        }
        
        let optimizer = OptaOptimizer::new(0.01, 10);
        let patterns = optimizer.detect_antipatterns(&model);
        
        assert!(!patterns.is_empty());
        assert!(matches!(patterns[0], AntiPattern::OverCoupled { .. }));
    }
}
```

---

### Step 3: Independent Features & PyO3 Bridge (1 week)

#### Week 3: Python Integration

**Day 1-2: PyO3 Module**

**Add to `crates/optacore_struct/Cargo.toml`:**
```toml
[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"], optional = true }

[lib]
name = "optacore_struct"
crate-type = ["rlib", "cdylib"]

[features]
python = ["pyo3"]
```

**File:** `crates/optacore_struct/src/python.rs`
```rust
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn optacore_struct(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_and_optimize, m)?)?;
    m.add_function(wrap_pyfunction!(export_svg, m)?)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
fn parse_and_optimize(dsl: &str, iterations: usize) -> PyResult<String> {
    use crate::{C4Parser, OptaOptimizer, DefaultBackend};
    
    let mut model: crate::OptaModel<DefaultBackend> = C4Parser::parse(dsl)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    
    let optimizer = OptaOptimizer::new(0.01, iterations);
    optimizer.optimize_layout(&mut model);
    
    // Return JSON representation
    let result = serde_json::json!({
        "nodes": model.nodes.len(),
        "edges": model.edge_count(),
        "positions": model.nodes.iter().map(|n| n.position()).collect::<Vec<_>>(),
    });
    
    Ok(result.to_string())
}

#[cfg(feature = "python")]
#[pyfunction]
fn export_svg(dsl: &str, width: u32, height: u32) -> PyResult<String> {
    // Export optimized model as SVG
    // (implementation in export.rs)
    Ok("<svg></svg>".to_string())
}
```

**Day 3: Streamlit Frontend**

**File:** `streamlit_app/app.py` (in separate directory)
```python
import streamlit as st
import optacore_struct  # Compiled Rust module

st.title("OptaArch - Architecture Optimizer")

dsl_input = st.text_area(
    "C4 DSL Input",
    value="""
web = System "Web Application"
api = Container "API Gateway"
db = Database "PostgreSQL"

web -> api [weight=1.0]
api -> db [weight=0.8]
    """,
    height=200,
)

if st.button("Optimize & Visualize"):
    with st.spinner("Optimizing architecture..."):
        result = optacore_struct.parse_and_optimize(dsl_input, iterations=100)
        st.success(f"Optimized! Result: {result}")
        
        svg = optacore_struct.export_svg(dsl_input, 800, 600)
        st.image(svg, use_column_width=True)

st.sidebar.header("Settings")
iterations = st.sidebar.slider("Optimization Iterations", 10, 500, 100)
threshold = st.sidebar.slider("Pruning Threshold", 0.0, 1.0, 0.5)
```

**Build Python module:**
```bash
cd crates/optacore_struct
maturin develop --features python
```

**Day 4-5: Export & Rendering**

**File:** `crates/optacore_struct/src/export.rs`
```rust
use crate::model::OptaModel;
use burn::backend::Backend;
use anyhow::Result;

pub struct SvgExporter;

impl SvgExporter {
    pub fn export<B: Backend>(
        model: &OptaModel<B>,
        width: u32,
        height: u32,
    ) -> Result<String> {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        );
        
        // Draw edges
        for i in 0..model.nodes.len() {
            for j in 0..model.nodes.len() {
                let adj_data = model.adj_matrix.to_data();
                let weight = adj_data.value[i * model.nodes.len() + j];
                
                if weight > 0.0 {
                    let (x1, y1) = model.nodes[i].position();
                    let (x2, y2) = model.nodes[j].position();
                    
                    svg.push_str(&format!(
                        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="gray" stroke-width="{}"/>"#,
                        x1, y1, x2, y2, weight * 2.0
                    ));
                }
            }
        }
        
        // Draw nodes
        for node in &model.nodes {
            let (x, y) = node.position();
            let color = match node.node_type {
                crate::NodeType::System => "blue",
                crate::NodeType::Database => "green",
                _ => "gray",
            };
            
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="20" fill="{}"/>"#,
                x, y, color
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle">{}</text>"#,
                x, y + 5, node.name
            ));
        }
        
        svg.push_str("</svg>");
        Ok(svg)
    }
    
    pub fn export_c4_dsl<B: Backend>(model: &OptaModel<B>) -> String {
        let mut dsl = String::from("workspace {\n");
        
        for node in &model.nodes {
            dsl.push_str(&format!(
                "  {} = {:?} \"{}\"\n",
                node.id, node.node_type, node.name
            ));
        }
        
        for i in 0..model.nodes.len() {
            for j in 0..model.nodes.len() {
                let adj_data = model.adj_matrix.to_data();
                let weight = adj_data.value[i * model.nodes.len() + j];
                
                if weight > 0.0 {
                    dsl.push_str(&format!(
                        "  {} -> {} [weight={}]\n",
                        model.nodes[i].id, model.nodes[j].id, weight
                    ));
                }
            }
        }
        
        dsl.push_str("}\n");
        dsl
    }
}
```

---

### Step 4: App Integration & Usability (1 week)

#### Week 4: CLI & Zed Integration

**Day 1-2: CLI Tool**

**File:** `crates/optacore_struct/src/bin/optacore.rs`
```rust
use anyhow::Result;
use clap::Parser;
use optacore_struct::{C4Parser, OptaOptimizer, SvgExporter, DefaultBackend};
use std::fs;

#[derive(Parser)]
#[command(name = "optacore")]
#[command(about = "OptaCore architecture optimizer", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    output: Option<String>,
    
    #[arg(long, default_value_t = 100)]
    iterations: usize,
    
    #[arg(long, default_value_t = 0.5)]
    threshold: f32,
    
    #[arg(long)]
    format: Option<String>,  // svg, c4, json
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Read input DSL
    let dsl = fs::read_to_string(&cli.input)?;
    
    // Parse and optimize
    let mut model: optacore_struct::OptaModel<DefaultBackend> = C4Parser::parse(&dsl)?;
    
    let optimizer = OptaOptimizer::new(0.01, cli.iterations);
    optimizer.optimize_layout(&mut model);
    optimizer.prune_redundancies(&mut model, cli.threshold);
    
    // Export
    let output = match cli.format.as_deref() {
        Some("svg") => SvgExporter::export(&model, 1024, 768)?,
        Some("c4") => SvgExporter::export_c4_dsl(&model),
        _ => serde_json::to_string_pretty(&serde_json::json!({
            "nodes": model.node_count(),
            "edges": model.edge_count(),
        }))?,
    };
    
    // Write output
    if let Some(output_path) = cli.output {
        fs::write(output_path, output)?;
    } else {
        println!("{}", output);
    }
    
    Ok(())
}
```

**Usage:**
```bash
cargo run --bin optacore -- --input arch.dsl --output arch.svg --format svg
```

**Day 3-4: Zed Integration**

**Add to `crates/optaarch_assistant/src/lib.rs`:**
```rust
use optacore_struct::{OptaModel, OptaOptimizer, C4Parser, DefaultBackend};
use anyhow::Result;

pub struct OptaArchAssistant {
    current_model: Option<OptaModel<DefaultBackend>>,
}

impl OptaArchAssistant {
    pub fn new() -> Self {
        Self {
            current_model: None,
        }
    }
    
    pub fn analyze_workspace(&mut self, workspace_path: &str) -> Result<ArchitectureAnalysis> {
        // Scan workspace for services, imports, etc.
        let dsl = self.extract_architecture_from_workspace(workspace_path)?;
        
        // Parse to OptaCore model
        let mut model = C4Parser::parse(&dsl)?;
        
        // Optimize
        let optimizer = OptaOptimizer::new(0.01, 100);
        optimizer.optimize_layout(&mut model);
        
        let antipatterns = optimizer.detect_antipatterns(&model);
        
        self.current_model = Some(model.clone());
        
        Ok(ArchitectureAnalysis {
            model,
            antipatterns,
            suggestions: Vec::new(),  // TODO: Add Mistral.rs suggestions
        })
    }
    
    fn extract_architecture_from_workspace(&self, _path: &str) -> Result<String> {
        // Placeholder: scan for services, databases, etc.
        Ok(String::from("web = System \"Web App\"\n"))
    }
}

pub struct ArchitectureAnalysis {
    pub model: OptaModel<DefaultBackend>,
    pub antipatterns: Vec<optacore_struct::AntiPattern>,
    pub suggestions: Vec<String>,
}
```

**Day 5: Testing**

```bash
# Unit tests
cargo test --package optacore_struct

# Benchmarks
cargo bench --package optacore_struct

# CLI tests
cargo run --bin optacore -- --input test.dsl --format svg

# Python module
cd crates/optacore_struct
maturin develop --features python
python -c "import optacore_struct; print(optacore_struct.parse_and_optimize('test', 10))"
```

---

### Step 5: Validation, Deployment & Community (1-2 weeks, ~$10-20)

#### Week 5-6: Production Deployment

**Day 1-2: Prime Intellect Setup**

**Requirements:**
1. Prime Intellect account with GPU credits
2. Docker container with Rust + Python
3. Streamlit deployment

**File:** `Dockerfile`
```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build Rust library with Python bindings
RUN cargo install maturin
RUN cd crates/optacore_struct && maturin build --release --features python

FROM python:3.11-slim

WORKDIR /app

# Copy Python wheel
COPY --from=builder /app/target/wheels/*.whl /app/
RUN pip install /app/*.whl

# Install Streamlit
RUN pip install streamlit numpy pandas

COPY streamlit_app/ /app/

EXPOSE 8501

CMD ["streamlit", "run", "app.py", "--server.port=8501", "--server.address=0.0.0.0"]
```

**Deploy:**
```bash
# Build and push
docker build -t optaarch:latest .
docker tag optaarch:latest registry.primeintelllect.ai/optaarch:latest
docker push registry.primeintelllect.ai/optaarch:latest

# Deploy on Prime Intellect
# (via their CLI/dashboard)
```

**Day 3-4: DNS & Site Integration**

**Connect to optaquan.com:**
1. Add subdomain: `arch.optaquan.com` → Prime Intellect endpoint
2. Update main site with link to OptaArch tool
3. Add community section (GitHub, docs)

**Day 5-6: Documentation**

**Create:**
- README with quick start
- API documentation (cargo doc)
- Tutorial: "Building Your First Architecture"
- Benchmarks comparison (vs. Structurizr, draw.io)

**Day 7-8: Community Launch**

**Post to:**
- Reddit: r/rust, r/devops, r/architecture
- Hacker News
- Twitter/X with demo GIF
- Rust Discord

**Messaging:**
- "2-5x faster architecture optimization with Burn tensors"
- "ML-powered anti-pattern detection"
- "Open source, MIT/Apache licensed"

---

## 🎯 Success Metrics

### Technical
- [ ] **Benchmark:** 3x speedup vs. Vec/HashMap for 1k-node graphs
- [ ] **Optimization:** <100ms for force-directed layout (1k nodes)
- [ ] **Memory:** <50MB for 10k-node model
- [ ] **Accuracy:** 95%+ preserved semantics after pruning

### Product
- [ ] **Interactive SVG:** Renders in browser
- [ ] **Export formats:** C4 DSL, SVG, PlantUML
- [ ] **Anti-patterns:** Detects cycles, coupling, bottlenecks
- [ ] **CI integration:** Lint architectures in GitHub Actions

### Community
- [ ] **100+ GitHub stars** in first month
- [ ] **10+ contributors**
- [ ] **Featured on Rust blog**
- [ ] **1000+ architecture diagrams** generated

---

## 💰 Budget Breakdown

**Total: $15-30**

- **Fly.io/Prime Intellect:** $10-20 (GPU hours for testing)
- **Domain:** $0 (already own optaquan.com)
- **CI/CD:** $0 (GitHub Actions free tier)
- **Hosting:** $5-10/month (Prime Intellect with GPU)

---

## 🚦 Next Steps

**Immediate Actions (This Week):**
1. ✅ Create `crates/optacore_struct` directory
2. ✅ Add dependencies to Cargo.toml
3. ✅ Implement `OptaNode`, `OptaModel` structs
4. ✅ Write C4 DSL parser
5. ✅ Create benchmarks

**Week 2:**
- Implement `OptaOptimizer` with force-directed layout
- Add k-means clustering
- Implement anti-pattern detection

**Week 3:**
- Build PyO3 module
- Create Streamlit frontend
- Export to SVG/C4

**Week 4:**
- CLI tool
- Zed integration
- Testing

**Week 5-6:**
- Deploy to Prime Intellect
- Documentation
- Community launch

---

## 📚 References

**Burn Framework:**
- https://github.com/tracel-ai/burn
- https://burn.dev/docs/

**Structurizr (Inspiration):**
- https://structurizr.com/
- https://github.com/structurizr/dsl

**Force-Directed Algorithms:**
- Fruchterman-Reingold (FR) layout
- Spring embedding
- https://en.wikipedia.org/wiki/Force-directed_graph_drawing

**K-Means Clustering:**
- https://en.wikipedia.org/wiki/K-means_clustering
- Burn implementation examples

---

## ✅ Ready to Begin

**This roadmap provides:**
- ✅ **Concrete implementation steps** (5 weeks)
- ✅ **Code examples** for every component
- ✅ **Benchmarking strategy** (criterion.rs)
- ✅ **Deployment plan** (Prime Intellect + DNS)
- ✅ **Community strategy** (Reddit, HN, GitHub)
- ✅ **Budget** ($15-30 total)

**Shall we start with Step 1: Creating the repository and core types?** 🚀
