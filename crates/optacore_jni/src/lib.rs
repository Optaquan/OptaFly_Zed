//! JNI Bridge for OptaCore
//!
//! Enables Java applications (especially Structurizr) to call Rust OptaCore functions
//! for C4 DSL parsing, layout optimization, and anti-pattern detection.
//!
//! ## Building
//! ```bash
//! cargo build --release --package optacore_jni
//! # Output: target/release/liboptacore_jni.{so,dylib,dll}
//! ```
//!
//! ## Java Integration
//! ```java
//! public class OptaCoreJNI {
//!     static { System.loadLibrary("optacore_jni"); }
//!     public static native String parseDsl(String dslInput);
//!     public static native String optimizeLayout(String modelJson);
//!     public static native String detectAntiPatterns(String modelJson, String configJson);
//!     public static native String generateDot(String modelJson, String configJson);
//!     public static native String getVersion();
//! }
//! ```
//!
//! ## Memory Management
//! Java side should call `env.DeleteLocalRef(result)` if holding JNI strings long-term
//! to prevent rare leaks. For typical usage (immediate consumption), GC handles cleanup.
//!
//! ## Debug Logging
//! Set `OPTACORE_JNI_DEBUG=1` environment variable to enable detailed logging.

use burn::backend::ndarray::NdArray;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use optacore_struct::{
    detect_anti_patterns, parse_c4_dsl, to_dot, AntiPatternConfig, NodeType, OptaModel,
    OptaOptimizer,
};
use serde_json::json;
use std::panic::{self, AssertUnwindSafe};

type Backend = NdArray<f32>;

/// Check if debug logging is enabled via environment variable
fn is_debug_enabled() -> bool {
    std::env::var("OPTACORE_JNI_DEBUG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Debug log helper
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if is_debug_enabled() {
            eprintln!("[OptaCore JNI] {}", format!($($arg)*));
        }
    };
}

/// Validate AntiPatternConfig thresholds
fn validate_config(config: &AntiPatternConfig) -> anyhow::Result<()> {
    if config.bottleneck_threshold < 1 {
        anyhow::bail!(
            "Invalid bottleneck_threshold: {} (must be >= 1)",
            config.bottleneck_threshold
        );
    }
    if config.over_coupling_threshold < 1 {
        anyhow::bail!(
            "Invalid over_coupling_threshold: {} (must be >= 1)",
            config.over_coupling_threshold
        );
    }
    Ok(())
}

/// Helper: Convert Rust Result to JString or throw Java exception
fn result_to_jstring(env: &mut JNIEnv, result: anyhow::Result<String>) -> jstring {
    match result {
        Ok(value) => match env.new_string(&value) {
            Ok(s) => {
                debug_log!("Success: {} bytes", value.len());
                s.into_raw()
            }
            Err(e) => {
                let error_msg = format!("String conversion error: {}", e);
                debug_log!("Error: {}", error_msg);
                let _ = env.throw_new("java/lang/RuntimeException", error_msg);
                std::ptr::null_mut()
            }
        },
        Err(err) => {
            let error_msg = format!("OptaCore error: {}", err);
            debug_log!("Error: {}", error_msg);
            let _ = env.throw_new("java/lang/RuntimeException", error_msg);
            std::ptr::null_mut()
        }
    }
}

/// Catch panics and convert to Java exceptions
fn with_panic_handler<F>(env: &mut JNIEnv, method_name: &str, f: F) -> jstring
where
    F: FnOnce(&mut JNIEnv) -> anyhow::Result<String>,
{
    debug_log!("Entering {}", method_name);

    let env_ptr = env as *mut JNIEnv;
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let env = unsafe { &mut *env_ptr };
        f(env)
    }));

    match result {
        Ok(inner_result) => {
            debug_log!("Exiting {} (normal)", method_name);
            result_to_jstring(env, inner_result)
        }
        Err(panic_info) => {
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                format!("Rust panic in {}: {}", method_name, s)
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                format!("Rust panic in {}: {}", method_name, s)
            } else {
                format!("Rust panic in {}: unknown error", method_name)
            };
            debug_log!("PANIC: {}", panic_msg);
            let _ = env.throw_new("java/lang/RuntimeException", panic_msg);
            std::ptr::null_mut()
        }
    }
}

/// Parse C4 DSL and return JSON model
///
/// Java signature: `public static native String parseDsl(String dslInput);`
///
/// Returns JSON with nodes, edges, counts. Null on error (exception thrown).
#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_parseDsl(
    mut env: JNIEnv,
    _class: JClass,
    dsl_input: JString,
) -> jstring {
    with_panic_handler(&mut env, "parseDsl", |env| {
        let dsl: String = env
            .get_string(&dsl_input)
            .map_err(|e| anyhow::anyhow!("Invalid DSL input (UTF-8 error): {}", e))?
            .into();

        debug_log!("Parsing DSL ({} chars)", dsl.len());

        if dsl.trim().is_empty() {
            anyhow::bail!("DSL input is empty");
        }

        let model: OptaModel<Backend> = parse_c4_dsl::<Backend>(&dsl)?;

        debug_log!(
            "Parsed: {} nodes, {} edges",
            model.node_count(),
            model.edge_count()
        );

        let json_model = json!({
            "nodes": model.nodes.iter().map(|n| {
                json!({
                    "id": n.id,
                    "original_id": n.id,
                    "name": n.name,
                    "type": match n.node_type {
                        NodeType::System => "System",
                        NodeType::Container => "Container",
                        NodeType::Component => "Component",
                        NodeType::Person => "Person",
                    },
                    "position": n.get_position(),
                    "technology": n.technology,
                    "description": n.description,
                })
            }).collect::<Vec<_>>(),
            "edges": model.edges.iter().map(|e| json!({
                "from": e.from,
                "to": e.to,
                "label": e.label,
                "weight": e.weight,
            })).collect::<Vec<_>>(),
            "node_count": model.node_count(),
            "edge_count": model.edge_count(),
        });

        Ok(json_model.to_string())
    })
}

/// Optimize layout on JSON model input, return updated JSON
///
/// Java signature: `public static native String optimizeLayout(String modelJson);`
///
/// Returns JSON with optimized node positions + stats. Null on error.
#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_optimizeLayout(
    mut env: JNIEnv,
    _class: JClass,
    model_json: JString,
) -> jstring {
    with_panic_handler(&mut env, "optimizeLayout", |env| {
        let json_str: String = env
            .get_string(&model_json)
            .map_err(|e| anyhow::anyhow!("Invalid JSON input (UTF-8 error): {}", e))?
            .into();

        debug_log!("Deserializing model ({} chars)", json_str.len());

        if json_str.trim().is_empty() {
            anyhow::bail!("Model JSON is empty");
        }

        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| anyhow::anyhow!("JSON parse error: {}", e))?;

        let mut model = json_to_model(&json_value)?;

        debug_log!(
            "Optimizing layout for {} nodes, {} edges",
            model.node_count(),
            model.edge_count()
        );

        model.build_adjacency_matrix();

        let start = std::time::Instant::now();
        let optimizer = OptaOptimizer::<Backend>::new(150, 0.2).with_area(2000.0);
        optimizer.optimize_layout(&mut model)?;
        let duration_ms = start.elapsed().as_millis() as u64;

        debug_log!("Optimization complete: {}ms", duration_ms);

        let json_output = json!({
            "nodes": model.nodes.iter().map(|n| json!({
                "id": n.id,
                "original_id": n.id,
                "name": n.name,
                "type": match n.node_type {
                    NodeType::System => "System",
                    NodeType::Container => "Container",
                    NodeType::Component => "Component",
                    NodeType::Person => "Person",
                },
                "position": n.get_position(),
                "technology": n.technology,
                "description": n.description,
            })).collect::<Vec<_>>(),
            "edges": model.edges.iter().map(|e| json!({
                "from": e.from,
                "to": e.to,
                "label": e.label,
                "weight": e.weight,
            })).collect::<Vec<_>>(),
            "optimization_stats": {
                "iterations": 150,
                "duration_ms": duration_ms,
            },
        });

        Ok(json_output.to_string())
    })
}

/// Detect anti-patterns on JSON model, return patterns JSON
///
/// Java signature: `public static native String detectAntiPatterns(String modelJson, String configJson);`
///
/// Config can be empty string for defaults. Returns pattern list. Null on error.
#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_detectAntiPatterns(
    mut env: JNIEnv,
    _class: JClass,
    model_json: JString,
    config_json: JString,
) -> jstring {
    with_panic_handler(&mut env, "detectAntiPatterns", |env| {
        let json_str: String = env
            .get_string(&model_json)
            .map_err(|e| anyhow::anyhow!("Invalid model JSON (UTF-8 error): {}", e))?
            .into();

        if json_str.trim().is_empty() {
            anyhow::bail!("Model JSON is empty");
        }

        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| anyhow::anyhow!("Model JSON parse error: {}", e))?;

        let model = json_to_model(&json_value)?;

        let config_str: String = env
            .get_string(&config_json)
            .map_err(|e| anyhow::anyhow!("Invalid config JSON (UTF-8 error): {}", e))?
            .into();

        let config: AntiPatternConfig = if config_str.trim().is_empty() {
            debug_log!("Using default AntiPatternConfig");
            AntiPatternConfig::default()
        } else {
            debug_log!("Parsing custom config ({} chars)", config_str.len());
            let json: serde_json::Value = serde_json::from_str(&config_str)
                .map_err(|e| anyhow::anyhow!("Config JSON parse error: {}", e))?;

            let bottleneck = json
                .get("bottleneck_threshold")
                .and_then(|v| v.as_u64())
                .unwrap_or(5) as usize;
            let over_coupling = json
                .get("over_coupling_threshold")
                .and_then(|v| v.as_u64())
                .unwrap_or(8) as usize;

            let cfg = AntiPatternConfig {
                bottleneck_threshold: bottleneck,
                over_coupling_threshold: over_coupling,
                detect_isolated: true,
            };
            validate_config(&cfg)?;
            cfg
        };

        debug_log!(
            "Detecting patterns (bottleneck_threshold={}, over_coupling_threshold={})",
            config.bottleneck_threshold,
            config.over_coupling_threshold
        );

        let patterns = detect_anti_patterns(&model, &config)?;

        debug_log!("Found {} patterns", patterns.len());

        let json_patterns = json!({
            "patterns": patterns.iter().map(|p| {
                match p {
                    optacore_struct::AntiPattern::Cycle { nodes } => json!({
                        "type": "Cycle",
                        "nodes": nodes,
                        "severity": 1.0,
                        "description": format!("Circular dependency detected: {} nodes", nodes.len()),
                    }),
                    optacore_struct::AntiPattern::Bottleneck { node_id, in_degree, severity } => json!({
                        "type": "Bottleneck",
                        "node_id": node_id,
                        "in_degree": in_degree,
                        "severity": severity,
                        "description": format!("High fan-in: {} incoming edges", in_degree),
                    }),
                    optacore_struct::AntiPattern::IsolatedComponent { node_id } => json!({
                        "type": "IsolatedComponent",
                        "node_id": node_id,
                        "severity": 0.3,
                        "description": "No connections to other components",
                    }),
                    optacore_struct::AntiPattern::OverCoupling { node_id, out_degree, severity } => json!({
                        "type": "OverCoupling",
                        "node_id": node_id,
                        "out_degree": out_degree,
                        "severity": severity,
                        "description": format!("High fan-out: {} outgoing edges", out_degree),
                    }),
                }
            }).collect::<Vec<_>>(),
            "count": patterns.len(),
        });

        Ok(json_patterns.to_string())
    })
}

/// Generate Graphviz DOT visualization
///
/// Java signature: `public static native String generateDot(String modelJson, String configJson);`
///
/// Config can be empty for defaults. Returns DOT string ready for Graphviz. Null on error.
#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_generateDot(
    mut env: JNIEnv,
    _class: JClass,
    model_json: JString,
    config_json: JString,
) -> jstring {
    with_panic_handler(&mut env, "generateDot", |env| {
        let json_str: String = env
            .get_string(&model_json)
            .map_err(|e| anyhow::anyhow!("Invalid model JSON (UTF-8 error): {}", e))?
            .into();

        if json_str.trim().is_empty() {
            anyhow::bail!("Model JSON is empty");
        }

        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| anyhow::anyhow!("Model JSON parse error: {}", e))?;

        let model = json_to_model(&json_value)?;

        let config_str: String = env
            .get_string(&config_json)
            .map_err(|e| anyhow::anyhow!("Invalid config JSON (UTF-8 error): {}", e))?
            .into();

        let config: Option<AntiPatternConfig> = if config_str.trim().is_empty() {
            debug_log!("Using default config for DOT generation");
            Some(AntiPatternConfig::default())
        } else {
            debug_log!("Using custom config for DOT generation");
            let json: serde_json::Value = serde_json::from_str(&config_str)
                .map_err(|e| anyhow::anyhow!("Config JSON parse error: {}", e))?;

            let bottleneck = json
                .get("bottleneck_threshold")
                .and_then(|v| v.as_u64())
                .unwrap_or(5) as usize;
            let over_coupling = json
                .get("over_coupling_threshold")
                .and_then(|v| v.as_u64())
                .unwrap_or(8) as usize;

            let cfg = AntiPatternConfig {
                bottleneck_threshold: bottleneck,
                over_coupling_threshold: over_coupling,
                detect_isolated: true,
            };
            validate_config(&cfg)?;
            Some(cfg)
        };

        debug_log!("Generating DOT for {} nodes", model.node_count());

        let dot = to_dot(&model, config.as_ref())?;

        debug_log!("Generated DOT ({} bytes)", dot.len());

        Ok(dot)
    })
}

/// Get version information
///
/// Java signature: `public static native String getVersion();`
///
/// Returns version string (e.g., "0.1.0"). Never fails.
#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_getVersion(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    with_panic_handler(&mut env, "getVersion", |_env| {
        Ok(env!("CARGO_PKG_VERSION").to_string())
    })
}

/// Helper: Convert JSON value to OptaModel
fn json_to_model(json: &serde_json::Value) -> anyhow::Result<OptaModel<Backend>> {
    use optacore_struct::model::{OptaEdge, OptaNode};

    let nodes_array = json
        .get("nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Missing 'nodes' array"))?;

    let edges_array = json
        .get("edges")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Missing 'edges' array"))?;

    let mut model = OptaModel::<Backend>::new();

    for node_val in nodes_array {
        let id = node_val
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Node missing 'id'"))?
            .to_string();

        let name = node_val
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let node_type_str = node_val
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("System");

        let node_type = match node_type_str {
            "Container" => NodeType::Container,
            "Component" => NodeType::Component,
            "Person" => NodeType::Person,
            _ => NodeType::System,
        };

        let technology = node_val
            .get("technology")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let description = node_val
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut node = OptaNode::new(id, name, node_type);
        node.technology = technology;
        node.description = description;

        if let Some(pos) = node_val.get("position").and_then(|v| v.as_array()) {
            if pos.len() == 2 {
                if let (Some(x), Some(y)) = (pos[0].as_f64(), pos[1].as_f64()) {
                    node.set_position(x as f32, y as f32);
                }
            }
        }

        model.add_node(node);
    }

    for edge_val in edges_array {
        let from = edge_val
            .get("from")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Edge missing 'from'"))?
            .to_string();

        let to = edge_val
            .get("to")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Edge missing 'to'"))?
            .to_string();

        let label = edge_val
            .get("label")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let weight = edge_val
            .get("weight")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0) as f32;

        model.add_edge(OptaEdge {
            from,
            to,
            label,
            weight,
        });
    }

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_available() {
        let _device = <Backend as burn::tensor::backend::Backend>::Device::default();
    }

    #[test]
    fn test_config_validation() {
        let valid = AntiPatternConfig {
            bottleneck_threshold: 5,
            over_coupling_threshold: 8,
            detect_isolated: true,
        };
        assert!(validate_config(&valid).is_ok());

        let invalid_bottleneck = AntiPatternConfig {
            bottleneck_threshold: 0,
            over_coupling_threshold: 8,
            detect_isolated: true,
        };
        assert!(validate_config(&invalid_bottleneck).is_err());
    }

    #[test]
    fn test_debug_enabled() {
        std::env::remove_var("OPTACORE_JNI_DEBUG");
        assert!(!is_debug_enabled());

        std::env::set_var("OPTACORE_JNI_DEBUG", "1");
        assert!(is_debug_enabled());

        std::env::set_var("OPTACORE_JNI_DEBUG", "true");
        assert!(is_debug_enabled());

        std::env::set_var("OPTACORE_JNI_DEBUG", "0");
        assert!(!is_debug_enabled());
    }
}

/// Health check - verifies Rust backend initialization
///
/// Java signature: `public static native boolean healthCheck();`
///
/// Returns true if OptaCore is functional (backend available). Never fails.
#[no_mangle]
pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_healthCheck(
    _env: JNIEnv,
    _class: JClass,
) -> jni::sys::jboolean {
    // Test backend availability
    match std::panic::catch_unwind(|| {
        let _device = <Backend as burn::tensor::backend::Backend>::Device::default();
        true
    }) {
        Ok(_) => jni::sys::JNI_TRUE,
        Err(_) => jni::sys::JNI_FALSE,
    }
}
