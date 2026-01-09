# OptaCore Integration & ML Roadmap

## Overview

This document outlines three integration paths:
1. **Structurizr Plugin** - Easy integration with existing C4 tooling
2. **Neural Position Predictor** - First ML model for learned layout
3. **CLI Tool** - Standalone executable for immediate usage

---

## 1. Structurizr Integration

### Option A: Inline Script (Easy, 1 day)

**File:** `optafly.kts` (Kotlin script)

```kotlin
// Run in Structurizr DSL: !script optafly.kts

import com.structurizr.model.*
import com.structurizr.view.*
import java.io.File
import java.nio.file.Files
import java.nio.file.Paths
import org.json.JSONObject

// Run OptaFly CLI
val process = Runtime.getRuntime().exec("optafly parse input.c4 --output model.json")
process.waitFor()

// Read OptaFly JSON
val jsonStr = Files.readString(Paths.get("model.json"))
val json = JSONObject(jsonStr)

// Map to Structurizr model
val softwareSystem = workspace.model.addSoftwareSystem("My System", "AI-Optimized")

json.getJSONArray("nodes").forEach { nodeObj ->
    val node = nodeObj as JSONObject
    val container = softwareSystem.addContainer(
        node.getString("name"),
        "Auto-generated",
        node.getString("technology")
    )
    
    // Import optimized positions
    if (node.has("position")) {
        val pos = node.getJSONObject("position")
        container.setProperty("x", pos.getDouble("x").toString())
        container.setProperty("y", pos.getDouble("y").toString())
    }
}

json.getJSONArray("edges").forEach { edgeObj ->
    val edge = edgeObj as JSONObject
    val from = softwareSystem.getContainerWithName(edge.getString("from"))
    val to = softwareSystem.getContainerWithName(edge.getString("to"))
    from?.uses(to, edge.getString("label"))
}

// Import anti-patterns as tags
json.getJSONArray("patterns").forEach { patternObj ->
    val pattern = patternObj as JSONObject
    val type = pattern.getString("type")
    val severity = pattern.getDouble("severity")
    
    when (type) {
        "Cycle" -> pattern.getJSONArray("nodes").forEach { nodeId ->
            val container = softwareSystem.getContainerWithName(nodeId.toString())
            container?.addTags("cycle", "critical")
        }
        "Bottleneck" -> {
            val container = softwareSystem.getContainerWithName(pattern.getString("node_id"))
            container?.addTags("bottleneck", if (severity > 1.0) "critical" else "warning")
        }
    }
}

// Create view with optimized layout
val view = workspace.views.createContainerView(softwareSystem, "optimized", "AI-Optimized Layout")
view.enableAutomaticLayout(false) // Use OptaFly positions

File("model.json").delete()
```

**Benefits:**
- Zero code changes to OptaFly
- Works with existing Structurizr installations
- Users can embed: `!script optafly.kts` in their DSL

**Limitations:**
- Requires OptaFly CLI installed
- Limited error handling
- Can't customize optimization parameters inline

### Option B: Full Java Plugin (Advanced, 3-5 days)

**Architecture:**
```
com.optafly.structurizr/
├── OptaFlyPlugin.java (implements WorkspaceWriterPlugin)
├── OptaCoreJNI.java (JNI bridge to Rust)
└── native/
    └── liboptacore_jni.so (Rust dylib with JNI exports)
```

**Rust JNI Bridge** (`crates/optacore_jni/`):

```rust
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use optacore_struct::{parse_c4_dsl, OptaOptimizer, AntiPatternConfig, detect_anti_patterns};
use burn::backend::ndarray::NdArray;

#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_optimizeLayout(
    env: JNIEnv,
    _class: JClass,
    dsl_input: JString,
) -> jstring {
    let dsl: String = env.get_string(dsl_input).expect("Invalid DSL").into();
    
    let mut model = parse_c4_dsl::<NdArray<f32>>(&dsl).unwrap();
    model.build_adjacency_matrix();
    
    let optimizer = OptaOptimizer::new(150, 0.2);
    optimizer.optimize_layout(&mut model).unwrap();
    
    let config = AntiPatternConfig::default();
    let patterns = detect_anti_patterns(&model, &config).unwrap();
    
    let result = serde_json::json!({
        "nodes": model.nodes,
        "edges": model.edges,
        "patterns": patterns
    });
    
    let output = env.new_string(result.to_string()).unwrap();
    output.into_inner()
}
```

**Java Plugin:**

```java
public class OptaFlyPlugin implements WorkspaceWriterPlugin {
    static {
        System.loadLibrary("optacore_jni");
    }
    
    @Override
    public void run(Workspace workspace) {
        String dsl = exportToDSL(workspace);
        String resultJson = OptaCoreJNI.optimizeLayout(dsl);
        JSONObject result = new JSONObject(resultJson);
        
        // Apply optimized positions back to workspace
        applyPositions(workspace, result);
        applyAntiPatternTags(workspace, result);
    }
}
```

**Benefits:**
- Tight integration, no CLI needed
- Can expose all OptaCore configuration
- Performance (direct JNI, no subprocess)
- Can run in Structurizr cloud/on-prem

**Effort:** 
- Create `crates/optacore_jni` crate
- Implement JNI bindings
- Package as Maven artifact
- Test with Structurizr

---

## 2. Neural Position Predictor (Phase 3)

### Architecture

**Crate:** `crates/optacore_ml`

```
crates/optacore_ml/
├── src/
│   ├── lib.rs
│   ├── models/
│   │   ├── position_mlp.rs      // Simple MLP (this phase)
│   │   ├── position_gnn.rs      // Future: Graph Neural Network
│   │   └── layout_gat.rs        // Future: Graph Attention Network
│   ├── features.rs              // Feature extraction
│   ├── training.rs              // Training loop
│   └── inference.rs             // Prediction interface
└── Cargo.toml
```

### Phase 3.1: Simple MLP (Week 1-2)

**Model:** `position_mlp.rs`

```rust
use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig, Dropout, DropoutConfig},
    tensor::{backend::Backend, Tensor, activation::relu},
};

#[derive(Module, Debug)]
pub struct PositionMLP<B: Backend> {
    fc1: Linear<B>,
    dropout1: Dropout,
    fc2: Linear<B>,
    dropout2: Dropout,
    fc3: Linear<B>,
}

#[derive(Config, Debug)]
pub struct PositionMLPConfig {
    #[config(default = 50)]
    input_dim: usize,      // Node features
    #[config(default = 256)]
    hidden_dim1: usize,
    #[config(default = 128)]
    hidden_dim2: usize,
    #[config(default = 2)]
    output_dim: usize,     // (x, y) position
    #[config(default = 0.3)]
    dropout_rate: f64,
}

impl PositionMLPConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> PositionMLP<B> {
        PositionMLP {
            fc1: LinearConfig::new(self.input_dim, self.hidden_dim1).init(device),
            dropout1: DropoutConfig::new(self.dropout_rate).init(),
            fc2: LinearConfig::new(self.hidden_dim1, self.hidden_dim2).init(device),
            dropout2: DropoutConfig::new(self.dropout_rate).init(),
            fc3: LinearConfig::new(self.hidden_dim2, self.output_dim).init(device),
        }
    }
}

impl<B: Backend> PositionMLP<B> {
    /// Forward pass: node_features [batch, nodes, features] -> positions [batch, nodes, 2]
    pub fn forward(&self, features: Tensor<B, 3>) -> Tensor<B, 3> {
        let [batch, nodes, _] = features.dims();
        
        // Reshape to [batch * nodes, features]
        let x = features.reshape([batch * nodes, self.fc1.weight.dims()[1]]);
        
        // MLP layers
        let x = relu(self.fc1.forward(x));
        let x = self.dropout1.forward(x);
        let x = relu(self.fc2.forward(x));
        let x = self.dropout2.forward(x);
        let x = self.fc3.forward(x);
        
        // Reshape back to [batch, nodes, 2]
        x.reshape([batch, nodes, 2])
    }
}

// Parameter count: 50*256 + 256*128 + 128*2 = 45,824 params (~0.18 MB)
```

**Feature Extraction:** `features.rs`

```rust
use crate::{OptaModel, NodeType, AntiPattern, detect_anti_patterns, AntiPatternConfig};
use burn::tensor::{backend::Backend, Tensor};
use std::collections::HashMap;

/// Extract node features for ML model
pub fn extract_node_features<B: Backend>(
    model: &OptaModel<B>,
    patterns: &[AntiPattern],
) -> Tensor<B, 2> {
    let device = B::Device::default();
    let node_count = model.node_count();
    
    // Build degree and severity maps
    let in_degree = compute_in_degree(model);
    let out_degree = compute_out_degree(model);
    let severity_map = build_severity_map(patterns);
    
    let mut features = Vec::with_capacity(node_count * 50);
    
    for node in &model.nodes {
        // Feature vector (50-dim):
        
        // 1-5: Degree features
        let in_deg = in_degree.get(&node.id).copied().unwrap_or(0) as f32;
        let out_deg = out_degree.get(&node.id).copied().unwrap_or(0) as f32;
        let total_deg = in_deg + out_deg;
        let deg_ratio = if total_deg > 0.0 { in_deg / total_deg } else { 0.0 };
        features.extend_from_slice(&[in_deg, out_deg, total_deg, deg_ratio, total_deg.sqrt()]);
        
        // 6-9: Node type (one-hot)
        let type_vec = match node.node_type {
            NodeType::System => [1.0, 0.0, 0.0, 0.0],
            NodeType::Container => [0.0, 1.0, 0.0, 0.0],
            NodeType::Component => [0.0, 0.0, 1.0, 0.0],
            NodeType::Person => [0.0, 0.0, 0.0, 1.0],
        };
        features.extend_from_slice(&type_vec);
        
        // 10: Severity
        let severity = severity_map.get(&node.id).copied().unwrap_or(0.0);
        features.push(severity);
        
        // 11-14: Anti-pattern flags
        let is_in_cycle = is_node_in_cycle(&node.id, patterns);
        let is_bottleneck = is_bottleneck(&node.id, patterns);
        let is_isolated = is_isolated(&node.id, patterns);
        let is_over_coupled = is_over_coupled(&node.id, patterns);
        features.extend_from_slice(&[
            if is_in_cycle { 1.0 } else { 0.0 },
            if is_bottleneck { 1.0 } else { 0.0 },
            if is_isolated { 1.0 } else { 0.0 },
            if is_over_coupled { 1.0 } else { 0.0 },
        ]);
        
        // 15-19: Graph-level stats (normalized)
        let graph_size = node_count as f32;
        let edge_count = model.edge_count() as f32;
        let edge_density = edge_count / (graph_size * (graph_size - 1.0)).max(1.0);
        features.extend_from_slice(&[
            graph_size / 100.0,  // Normalize to ~[0, 1] for typical graphs
            edge_count / 100.0,
            edge_density,
            (edge_count / graph_size).min(10.0) / 10.0,
            0.0  // Placeholder for future: clustering coefficient
        ]);
        
        // 20-49: Neighbor embeddings (30 dims) - TODO: implement
        features.extend_from_slice(&[0.0; 30]);
    }
    
    Tensor::<B, 1>::from_floats(features.as_slice(), &device)
        .reshape([node_count, 50])
}
```

**Training:** `training.rs`

```rust
use burn::{
    optim::{AdamConfig, GradientsParams, Optimizer},
    tensor::{backend::AutodiffBackend, Tensor},
    train::{metric::LossMetric, TrainStep, ValidStep, TrainOutput},
};

pub struct PositionPredictorTrainer<B: AutodiffBackend> {
    model: PositionMLP<B>,
    optim: Adam<B>,
}

impl<B: AutodiffBackend> PositionPredictorTrainer<B> {
    pub fn train_step(&mut self, features: Tensor<B, 2>, targets: Tensor<B, 2>) -> f32 {
        // Forward pass
        let predictions = self.model.forward(features.clone().unsqueeze());
        
        // MSE loss
        let loss = (predictions - targets.unsqueeze()).powf_scalar(2.0).mean();
        
        // Backward pass
        let grads = loss.backward();
        let grads = GradientsParams::from_grads(grads, &self.model);
        self.model = self.optim.step(self.lr, self.model.clone(), grads);
        
        loss.into_scalar()
    }
}

// Training loop (simplified)
pub fn train_model<B: AutodiffBackend>(
    train_data: Vec<(Tensor<B, 2>, Tensor<B, 2>)>,
    epochs: usize,
) -> PositionMLP<B> {
    let config = PositionMLPConfig::new();
    let model = config.init(&B::Device::default());
    let mut trainer = PositionPredictorTrainer {
        model,
        optim: AdamConfig::new().init(),
    };
    
    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        for (features, targets) in &train_data {
            total_loss += trainer.train_step(features.clone(), targets.clone());
        }
        println!("Epoch {}: Loss = {:.4}", epoch, total_loss / train_data.len() as f32);
    }
    
    trainer.model
}
```

### Phase 3.2: Dataset Integration

**Usage with Synthetic Dataset:**

```rust
// In dataset_gen.rs, also save features + positions
let features = extract_node_features(&model, &patterns);
let positions: Vec<(f32, f32)> = model.nodes.iter()
    .map(|n| n.get_position().unwrap_or((0.0, 0.0)))
    .collect();

let data = json!({
    "features": features.to_data().value,  // Raw tensor data
    "positions": positions,
    // ... other fields
});
```

**Loading for Training:**

```rust
// Load dataset
let dataset_dir = PathBuf::from("dataset");
let mut train_data = Vec::new();

for entry in std::fs::read_dir(dataset_dir)? {
    let path = entry?.path();
    if path.extension() == Some("json") {
        let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        let features = Tensor::from_floats(
            json["features"].as_array().unwrap().iter()
                .map(|v| v.as_f64().unwrap() as f32).collect::<Vec<_>>().as_slice(),
            &device
        ).reshape([node_count, 50]);
        
        let positions = Tensor::from_floats(
            json["positions"].as_array().unwrap().iter().flat_map(|pair| {
                vec![pair[0].as_f64().unwrap() as f32, pair[1].as_f64().unwrap() as f32]
            }).collect::<Vec<_>>().as_slice(),
            &device
        ).reshape([node_count, 2]);
        
        train_data.push((features, positions));
    }
}

// Train
let model = train_model(train_data, epochs: 50);
```

### Phase 3.3: Hybrid Optimizer (Learned + Heuristic)

```rust
pub enum LayoutStrategy {
    HeuristicOnly,
    MLOnly,
    Hybrid { ml_weight: f32 },
}

impl OptaOptimizer {
    pub fn optimize_with_ml<B: Backend>(
        &self,
        model: &mut OptaModel<B>,
        ml_model: Option<&PositionMLP<B>>,
        strategy: LayoutStrategy,
    ) -> Result<OptimizationStats> {
        match strategy {
            LayoutStrategy::HeuristicOnly => self.optimize_layout(model),
            
            LayoutStrategy::MLOnly => {
                let features = extract_node_features(model, &[]);
                let predictions = ml_model.unwrap().forward(features.unsqueeze()).squeeze(0);
                apply_predictions_to_model(model, predictions)?;
                Ok(OptimizationStats { /* ... */ })
            },
            
            LayoutStrategy::Hybrid { ml_weight } => {
                // Get ML predictions as initial positions
                let features = extract_node_features(model, &[]);
                let ml_positions = ml_model.unwrap().forward(features.unsqueeze()).squeeze(0);
                apply_predictions_to_model(model, ml_positions)?;
                
                // Refine with fewer heuristic iterations
                let refined_optimizer = OptaOptimizer::new(
                    (self.iterations as f32 * (1.0 - ml_weight)) as usize,
                    self.learning_rate
                );
                refined_optimizer.optimize_layout(model)
            }
        }
    }
}
```

---

## 3. CLI Tool

**Implementation provided in your example** - ready to implement!

**Additional Commands to Consider:**

```bash
# Batch processing
optafly batch ./architectures/*.c4 --output-dir ./diagrams/

# Watch mode for live updates
optafly watch my-system.c4 --auto-render

# Compare layouts
optafly compare model1.json model2.json --metric edge-crossings

# Export to multiple formats
optafly export model.json --formats dot,svg,png,json

# ML commands (Phase 3)
optafly train --dataset ./dataset --epochs 50 --output model.burn
optafly predict model.json --model model.burn --output optimized.json
```

---

## Timeline

### Immediate (This Week)
1. ✅ Telemetry + Visualization (DONE)
2. CLI tool implementation (1-2 days)
3. Quality metrics (1-2 days)

### Week 2
1. Synthetic dataset generator (1 day)
2. Generate 10k samples (run overnight)
3. Structurizr script (basic version) (1 day)
4. Begin `optacore_ml` crate setup (2 days)

### Week 3-4
1. Implement Position MLP (3 days)
2. Training pipeline (2 days)
3. Initial model training on synthetic data (1 day)
4. Hybrid optimizer integration (2 days)
5. Evaluation metrics (1 day)

### Week 5+
1. WASM deployment with telemetry
2. Real user data collection
3. Fine-tune model with real data
4. A/B testing framework
5. Full Structurizr plugin (if needed)

---

## Success Metrics

**Phase 2.5 (Current):**
- ✅ Telemetry logging works
- ✅ Visualization matches C4 standards
- ✅ Anti-patterns correctly highlighted

**Phase 3 (ML):**
- ML model trains without errors
- Predictions reduce optimization iterations by >30%
- Hybrid approach beats heuristic-only on edge crossings
- Model size < 5MB for WASM deployment

**Phase 4 (Integration):**
- Structurizr users can run OptaFly via script
- CLI tool has >100 users
- Real telemetry data flowing
- Fine-tuned model shows improvement over synthetic-only

---

## Decision Matrix

| Feature | Priority | Effort | User Value | ML Value |
|---------|----------|--------|------------|----------|
| CLI Tool | HIGH | Low | ⭐⭐⭐⭐⭐ | ⭐⭐ (data collection) |
| Quality Metrics | HIGH | Medium | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| Dataset Generator | HIGH | Low | ⭐ | ⭐⭐⭐⭐⭐ |
| Position MLP | MEDIUM | High | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| Structurizr Script | MEDIUM | Low | ⭐⭐⭐⭐ | ⭐ |
| Structurizr Plugin | LOW | Very High | ⭐⭐⭐⭐⭐ | ⭐ |
| Hybrid Optimizer | MEDIUM | Medium | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

**Recommended Order:**
1. CLI Tool (quick win, enables everything)
2. Quality Metrics + Dataset Generator (ML foundation)
3. Position MLP + Training (first learned model)
4. Hybrid Optimizer (best of both worlds)
5. Structurizr Script (easy integration)
6. Full Plugin (only if high demand)
