use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::types::{ProxyRequest, ProxyResponse};

/// Convert Rust ProxyRequest to Python dict
pub fn request_to_pydict<'py>(py: Python<'py>, request: &ProxyRequest) -> Result<&'py PyDict> {
    let dict = PyDict::new(py);

    dict.set_item("prompt", &request.prompt)?;
    dict.set_item("api_key", &request.api_key)?;

    // Convert cache anchors
    let anchors: Vec<&PyDict> = request
        .cache_anchors
        .iter()
        .map(|anchor| {
            let anchor_dict = PyDict::new(py);
            anchor_dict.set_item("position", anchor.position).ok();
            anchor_dict.set_item("content", &anchor.content).ok();
            anchor_dict.set_item("priority", anchor.priority).ok();
            anchor_dict
                .set_item("estimated_tokens", anchor.estimated_tokens)
                .ok();
            anchor_dict
        })
        .collect();

    dict.set_item("cache_anchors", anchors)?;

    // Convert optimization settings
    let opt_dict = PyDict::new(py);
    opt_dict.set_item("remove_redundancy", request.optimization.remove_redundancy)?;
    opt_dict.set_item("compress_context", request.optimization.compress_context)?;
    opt_dict.set_item("split_into_chunks", request.optimization.split_into_chunks)?;
    dict.set_item("optimization", opt_dict)?;

    Ok(dict)
}

/// Convert Python dict to Rust ProxyResponse
pub fn pydict_to_response(_py: Python, dict: &PyDict) -> Result<ProxyResponse> {
    let cache_hit = dict.get_item("cache_hit")?.unwrap().extract::<bool>()?;

    let cache_status = if cache_hit {
        let response = dict.get_item("response")?.unwrap().extract::<String>()?;
        let latency_ms = dict.get_item("latency_ms")?.unwrap().extract::<u64>()?;
        let similarity = dict
            .get_item("similarity_score")?
            .unwrap()
            .extract::<f32>()?;

        crate::types::CacheStatus::Hit {
            response,
            latency_ms,
            similarity_score: similarity,
        }
    } else {
        crate::types::CacheStatus::Miss
    };

    let response_text = if let Some(text) = dict.get_item("response_text")? {
        Some(text.extract::<String>()?)
    } else {
        None
    };

    let tokens_used = dict.get_item("tokens_used")?.unwrap().extract::<usize>()?;
    let latency_ms = dict.get_item("latency_ms")?.unwrap().extract::<u64>()?;

    Ok(ProxyResponse {
        cache_status,
        response_text,
        tokens_used,
        latency_ms,
    })
}
