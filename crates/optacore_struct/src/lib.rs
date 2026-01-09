//! OptaCore-Struct: Tensor-native architecture modeling and optimization
//!
//! A sovereign, high-performance alternative inspired by Structurizr,
//! using Burn tensors for graph storage and ML-powered optimizations.
//!
//! ## Key Features
//! - **Tensor-Native Storage**: Adjacency matrices as Burn tensors for 2-5x faster graph queries
//! - **Force-Directed Layout**: Gradient descent optimization using Fruchterman-Reingold algorithm
//! - **Anti-Pattern Detection**: Cycles, over-coupling, bottlenecks, isolated components
//! - **C4 DSL Parser**: Structurizr-compatible syntax for architecture modeling
//! - **WASM-Ready**: Run entirely in-browser for privacy-first, offline-capable diagramming
//! - **GPU Acceleration**: Optional `wgpu` feature for large architecture models
//!
//! ## Architecture
//! OptaCore uses Burn's backend-agnostic tensor operations, enabling:
//! - **ndarray** (default): CPU-based, WASM-compatible, no external dependencies
//! - **wgpu** (optional): GPU acceleration for 10k+ node models
//!
//! ## Example
//! ```rust,ignore
//! use optacore_struct::{OptaModel, OptaNode, NodeType};
//! use burn::backend::ndarray::NdArray;
//!
//! type Backend = NdArray<f32>;
//!
//! let mut model = OptaModel::<Backend>::new();
//! model.add_node(OptaNode::new(
//!     "api".to_string(),
//!     "API Gateway".to_string(),
//!     NodeType::Container,
//! ));
//! model.build_adjacency_matrix();
//! ```
//!
//! ## WASM Usage
//! With the `wasm` feature enabled, OptaCore can run in browsers:
//! ```bash
//! cargo build --target wasm32-unknown-unknown --features wasm
//! wasm-bindgen target/wasm32-unknown-unknown/release/optacore_struct.wasm \
//!   --out-dir pkg --target web
//! ```

pub mod anti_patterns;
pub mod model;
pub mod optimizer;
pub mod parser;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(test)]
mod integration_tests;

pub use anti_patterns::{AntiPattern, AntiPatternConfig, detect_anti_patterns};
pub use model::{NodeType, OptaModel, OptaNode};
pub use optimizer::OptaOptimizer;
pub use parser::parse_c4_dsl;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[cfg(feature = "wgpu")]
    type GpuBackend = burn::backend::Wgpu;

    #[test]
    fn test_node_creation() {
        let node = OptaNode::<TestBackend>::new(
            "user_service".to_string(),
            "User Service".to_string(),
            NodeType::Container,
        );
        assert_eq!(node.id, "user_service");
        assert_eq!(node.name, "User Service");
    }
}
