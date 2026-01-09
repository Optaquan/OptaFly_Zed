# Phase 2b Implementation Plan
## PyO3 Integration & User Approval UI

**Start Date:** 2026-01-08  
**Duration:** 2-3 weeks  
**Status:** 🚧 In Progress

---

## 🎯 Phase 2b Objectives

1. **Complete PyO3 Bridge** - Actual Widget-Log Python integration
2. **User Approval UI** - GPUI-based prompt refinement panel
3. **Telemetry Collection** - Data gathering for Phase 2c training
4. **End-to-End Testing** - Full integration with Widget-Log proxy

---

## 📋 Implementation Tasks

### Week 1: PyO3 Bridge Completion

#### Task 1.1: Widget-Log Python Module Interface
**Priority:** Critical  
**Estimated Time:** 2-3 days

**Subtasks:**
- [ ] Create Python wrapper module for Widget-Log proxy
- [ ] Implement `process_prompt()` function
- [ ] Implement `get_cache_stats()` function
- [ ] Implement `health_check()` function
- [ ] Add proper error handling and logging

**Deliverable:** `widget-log/widget_log_proxy.py` with Rust-callable functions

#### Task 1.2: Complete PyO3 Bridge Implementation
**Priority:** Critical  
**Estimated Time:** 2 days

**Subtasks:**
- [ ] Implement actual Python module loading in `bridge.rs`
- [ ] Complete `call_proxy_python()` with real Widget-Log calls
- [ ] Add connection pooling for performance
- [ ] Implement retry logic for transient failures
- [ ] Add comprehensive error handling

**Deliverable:** Fully functional PyO3 bridge with Widget-Log

#### Task 1.3: Integration Testing
**Priority:** High  
**Estimated Time:** 1 day

**Subtasks:**
- [ ] Unit tests for PyO3 conversions
- [ ] Integration tests with mock Widget-Log
- [ ] End-to-end tests with actual Widget-Log proxy
- [ ] Performance benchmarking

**Deliverable:** Test suite with >80% coverage

---

### Week 2: User Approval UI

#### Task 2.1: GPUI Prompt Refinement Panel
**Priority:** High  
**Estimated Time:** 3-4 days

**Subtasks:**
- [ ] Create `PromptRefinementPanel` struct in Zed
- [ ] Implement side-by-side comparison view
- [ ] Add syntax highlighting for code snippets
- [ ] Implement diff highlighting for changes
- [ ] Add action buttons (Accept, Modify, Reject)

**Deliverable:** `crates/zed/src/ui/prompt_refinement_panel.rs`

#### Task 2.2: User Interaction Flow
**Priority:** High  
**Estimated Time:** 2 days

**Subtasks:**
- [ ] Integrate panel with Zed's modal system
- [ ] Implement keyboard shortcuts
- [ ] Add inline editing capabilities
- [ ] Implement confirmation dialog
- [ ] Add loading states

**Deliverable:** Interactive approval workflow

#### Task 2.3: Performance Metrics Display
**Priority:** Medium  
**Estimated Time:** 1-2 days

**Subtasks:**
- [ ] Create metrics display component
- [ ] Show estimated improvement percentage
- [ ] Display cache hit probability
- [ ] Add token count comparison
- [ ] Show historical statistics

**Deliverable:** Metrics UI in refinement panel

---

### Week 3: Telemetry & Polish

#### Task 3.1: Telemetry Collection System
**Priority:** High  
**Estimated Time:** 2-3 days

**Subtasks:**
- [ ] Create `TelemetryCollector` struct
- [ ] Implement event logging (routing decisions, cache outcomes)
- [ ] Add data export functionality
- [ ] Implement privacy controls
- [ ] Create anonymization utilities

**Deliverable:** `crates/telemetry/` crate

#### Task 3.2: End-to-End Integration
**Priority:** Critical  
**Estimated Time:** 2 days

**Subtasks:**
- [ ] Wire prompt refinement UI to IntegratedPromptManager
- [ ] Connect telemetry to all decision points
- [ ] Implement graceful degradation
- [ ] Add comprehensive logging
- [ ] Performance optimization

**Deliverable:** Complete integrated system

#### Task 3.3: Documentation & Testing
**Priority:** High  
**Estimated Time:** 1-2 days

**Subtasks:**
- [ ] Create user guide for prompt refinement
- [ ] Document telemetry data format
- [ ] Write integration test suite
- [ ] Performance benchmarking report
- [ ] Phase 2b summary document

**Deliverable:** Complete Phase 2b documentation

---

## 🏗️ Technical Implementation Details

### 1. Widget-Log Python Module Interface

**File:** `widget-log/widget_log_proxy.py`

```python
"""
Widget-Log Proxy Interface for Rust Integration
Exposes Widget-Log functionality to Rust via PyO3
"""

import json
import logging
from typing import Dict, Any, Optional

# Import Widget-Log components
from app.cache_agent import CacheAgent
from app.secure_proxy import ProxyServer

class WidgetLogInterface:
    """Python interface for Rust calls"""
    
    def __init__(self, config_path: str):
        self.cache_agent = CacheAgent()
        self.proxy = ProxyServer()
        logging.basicConfig(level=logging.INFO)
    
    def process_prompt(self, request: Dict[str, Any]) -> Dict[str, Any]:
        """
        Process a prompt through Widget-Log caching
        
        Args:
            request: {
                "prompt": str,
                "cache_anchors": List[Dict],
                "api_key": str,
                "optimization": Dict
            }
        
        Returns:
            {
                "cache_hit": bool,
                "response": Optional[str],
                "response_text": Optional[str],
                "tokens_used": int,
                "latency_ms": int,
                "similarity_score": Optional[float]
            }
        """
        try:
            prompt = request["prompt"]
            api_key = request["api_key"]
            
            # Check cache
            cache_result = self.cache_agent.check_cache(prompt)
            
            if cache_result["hit"]:
                return {
                    "cache_hit": True,
                    "response": cache_result["response"],
                    "response_text": cache_result["response"],
                    "tokens_used": 0,
                    "latency_ms": cache_result["latency_ms"],
                    "similarity_score": cache_result["similarity"]
                }
            
            # Forward to Anthropic API
            api_response = self.proxy.forward_to_anthropic(
                prompt, 
                api_key,
                cache_anchors=request.get("cache_anchors", [])
            )
            
            # Store in cache
            self.cache_agent.store_response(prompt, api_response)
            
            return {
                "cache_hit": False,
                "response": None,
                "response_text": api_response["content"],
                "tokens_used": api_response["usage"]["total_tokens"],
                "latency_ms": api_response["latency_ms"],
                "similarity_score": None
            }
            
        except Exception as e:
            logging.error(f"Error processing prompt: {e}")
            raise
    
    def get_cache_stats(self) -> Dict[str, Any]:
        """Get cache statistics"""
        return self.cache_agent.get_statistics()
    
    def health_check(self) -> Dict[str, Any]:
        """Check Widget-Log health"""
        return {
            "status": "healthy",
            "cache_agent": self.cache_agent.is_healthy(),
            "proxy": self.proxy.is_running()
        }

# Module-level instance
_interface: Optional[WidgetLogInterface] = None

def initialize(config_path: str) -> None:
    """Initialize Widget-Log interface"""
    global _interface
    _interface = WidgetLogInterface(config_path)

def process_prompt(request_json: str) -> str:
    """Process prompt (JSON string in/out for Rust)"""
    request = json.loads(request_json)
    result = _interface.process_prompt(request)
    return json.dumps(result)

def get_cache_stats() -> str:
    """Get cache stats (JSON string for Rust)"""
    stats = _interface.get_cache_stats()
    return json.dumps(stats)

def health_check() -> str:
    """Health check (JSON string for Rust)"""
    health = _interface.health_check()
    return json.dumps(health)
```

---

### 2. Enhanced PyO3 Bridge

**File:** `crates/pyo3_bridge/src/bridge.rs`

```rust
use anyhow::{Context, Result};
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::types::{ProxyRequest, ProxyResponse};

pub struct PyO3Bridge {
    widget_log_path: PathBuf,
    initialized: Arc<Mutex<bool>>,
}

impl PyO3Bridge {
    pub async fn new(widget_log_path: &str) -> Result<Self> {
        log::info!("Initializing PyO3 bridge for Widget-Log");
        
        let path = PathBuf::from(widget_log_path);
        
        if !path.exists() {
            anyhow::bail!("Widget-Log directory not found: {}", path.display());
        }
        
        Ok(Self {
            widget_log_path: path,
            initialized: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Initialize Widget-Log Python module
    pub async fn initialize(&self) -> Result<()> {
        let mut init = self.initialized.lock().await;
        if *init {
            return Ok(());
        }
        
        let widget_log_path = self.widget_log_path.clone();
        
        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                // Add Widget-Log to Python path
                let sys = py.import("sys")?;
                let sys_path = sys.getattr("path")?;
                sys_path.call_method1(
                    "insert", 
                    (0, widget_log_path.to_str().unwrap())
                )?;
                
                // Import and initialize Widget-Log module
                let widget_log = PyModule::import(py, "widget_log_proxy")?;
                
                let config_path = widget_log_path.join("config.yaml");
                widget_log.getattr("initialize")?.call1((
                    config_path.to_str().unwrap(),
                ))?;
                
                log::info!("Widget-Log Python module initialized");
                Ok::<(), anyhow::Error>(())
            })
        })
        .await??;
        
        *init = true;
        Ok(())
    }
    
    /// Send refined prompt to Widget-Log proxy
    pub async fn send_to_proxy(&self, request: ProxyRequest) -> Result<ProxyResponse> {
        // Ensure initialized
        self.initialize().await?;
        
        log::debug!("Sending request to Widget-Log proxy via PyO3");
        
        let widget_log_path = self.widget_log_path.clone();
        let response = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                Self::call_widget_log_python(py, &widget_log_path, request)
            })
        })
        .await
        .context("Failed to spawn blocking task")?
        .context("Python call failed")?;
        
        Ok(response)
    }
    
    /// Internal Python call (runs with GIL)
    fn call_widget_log_python(
        py: Python,
        _widget_log_path: &PathBuf,
        request: ProxyRequest,
    ) -> Result<ProxyResponse> {
        // Import module
        let widget_log = PyModule::import(py, "widget_log_proxy")?;
        
        // Serialize request to JSON
        let request_json = serde_json::to_string(&request)?;
        let request_py = PyString::new(py, &request_json);
        
        // Call process_prompt
        let result_py = widget_log
            .getattr("process_prompt")?
            .call1((request_py,))?;
        
        // Deserialize response
        let result_str: String = result_py.extract()?;
        let response: ProxyResponse = serde_json::from_str(&result_str)?;
        
        Ok(response)
    }
    
    /// Get cache statistics from Widget-Log
    pub async fn get_cache_stats(&self) -> Result<serde_json::Value> {
        self.initialize().await?;
        
        let widget_log_path = self.widget_log_path.clone();
        
        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                let widget_log = PyModule::import(py, "widget_log_proxy")?;
                
                let stats_py = widget_log.getattr("get_cache_stats")?.call0()?;
                let stats_str: String = stats_py.extract()?;
                let stats: serde_json::Value = serde_json::from_str(&stats_str)?;
                
                Ok(stats)
            })
        })
        .await?
    }
}
```

---

### 3. GPUI Prompt Refinement Panel

**File:** `crates/zed/src/ui/prompt_refinement_panel.rs`

```rust
use gpui::*;
use std::sync::Arc;

pub struct PromptRefinementPanel {
    original_prompt: SharedString,
    refined_prompt: SharedString,
    changes: Vec<PromptChange>,
    quality_score: f32,
    cache_improvement: f32,
    on_accept: Arc<dyn Fn(&WindowContext) + Send + Sync>,
    on_reject: Arc<dyn Fn(&WindowContext) + Send + Sync>,
}

#[derive(Clone)]
pub struct PromptChange {
    pub change_type: String,
    pub description: String,
    pub tokens_added: usize,
}

impl PromptRefinementPanel {
    pub fn new(
        original: String,
        refined: String,
        changes: Vec<PromptChange>,
        quality_score: f32,
        cache_improvement: f32,
        on_accept: impl Fn(&WindowContext) + Send + Sync + 'static,
        on_reject: impl Fn(&WindowContext) + Send + Sync + 'static,
    ) -> Self {
        Self {
            original_prompt: original.into(),
            refined_prompt: refined.into(),
            changes,
            quality_score,
            cache_improvement,
            on_accept: Arc::new(on_accept),
            on_reject: Arc::new(on_reject),
        }
    }
}

impl Render for PromptRefinementPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x1e1e1e))
            .border_1()
            .border_color(rgb(0x3e3e3e))
            .p_4()
            .gap_4()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Prompt Management Agent Suggestions")
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(0x2a4a2a))
                                    .rounded_md()
                                    .text_sm()
                                    .text_color(rgb(0x88ff88))
                                    .child(format!("+{:.0}% quality", self.quality_score * 100.0))
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(0x2a3a4a))
                                    .rounded_md()
                                    .text_sm()
                                    .text_color(rgb(0x88ccff))
                                    .child(format!("+{:.0}% cache hit", self.cache_improvement * 100.0))
                            )
                    )
            )
            .child(
                // Side-by-side comparison
                div()
                    .flex()
                    .gap_4()
                    .h(px(400.))
                    .child(
                        // Original prompt
                        div()
                            .flex_1()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xcccccc))
                                    .child("Original Prompt")
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .bg(rgb(0x2d2d2d))
                                    .rounded_md()
                                    .p_3()
                                    .overflow_y_scroll()
                                    .text_sm()
                                    .text_color(rgb(0xd4d4d4))
                                    .child(self.original_prompt.clone())
                            )
                    )
                    .child(
                        // Refined prompt
                        div()
                            .flex_1()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xcccccc))
                                    .child("Refined Prompt")
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .bg(rgb(0x1e3a1e))
                                    .border_1()
                                    .border_color(rgb(0x3a5a3a))
                                    .rounded_md()
                                    .p_3()
                                    .overflow_y_scroll()
                                    .text_sm()
                                    .text_color(rgb(0xe0e0e0))
                                    .child(self.refined_prompt.clone())
                            )
                    )
            )
            .child(
                // Changes summary
                div()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0xcccccc))
                            .child("Changes Applied:")
                    )
                    .children(self.changes.iter().map(|change| {
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(4.))
                                    .h(px(4.))
                                    .bg(rgb(0x88ff88))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xd4d4d4))
                                    .child(format!("{} (+{} tokens)", 
                                        change.description, 
                                        change.tokens_added))
                            )
                    }))
            )
            .child(
                // Action buttons
                div()
                    .flex()
                    .justify_end()
                    .gap_3()
                    .mt_4()
                    .child(
                        button()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x3a3a3a))
                            .rounded_md()
                            .text_sm()
                            .text_color(rgb(0xcccccc))
                            .hover(|style| style.bg(rgb(0x4a4a4a)))
                            .child("Use Original")
                            .on_click({
                                let on_reject = self.on_reject.clone();
                                move |_event, window, _cx| {
                                    on_reject(window);
                                }
                            })
                    )
                    .child(
                        button()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x2a5a2a))
                            .rounded_md()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .hover(|style| style.bg(rgb(0x3a7a3a)))
                            .child("Accept & Send")
                            .on_click({
                                let on_accept = self.on_accept.clone();
                                move |_event, window, _cx| {
                                    on_accept(window);
                                }
                            })
                    )
            )
    }
}
```

---

## 📊 Success Criteria

### Week 1 (PyO3 Integration)
- [ ] Widget-Log Python module callable from Rust
- [ ] Successful cache HIT/MISS detection
- [ ] Performance within 5ms of Phase 2a baseline
- [ ] Error handling for all failure modes

### Week 2 (User UI)
- [ ] Functional prompt refinement panel in Zed
- [ ] User can accept/reject refinements
- [ ] Side-by-side diff view working
- [ ] Metrics display accurate

### Week 3 (Polish & Testing)
- [ ] Telemetry collecting all decision points
- [ ] End-to-end tests passing
- [ ] Performance benchmarks documented
- [ ] User guide complete

---

## 🔄 Testing Strategy

### Unit Tests
- PyO3 type conversions
- Telemetry event logging
- UI component rendering

### Integration Tests
- Rust → Python → Widget-Log flow
- User approval workflow
- Telemetry data export

### End-to-End Tests
- Complete user journey
- Cache HIT/MISS scenarios
- Error recovery
- Performance under load

---

## 📈 Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| PyO3 overhead | <5ms | Direct measurement |
| UI render time | <16ms (60fps) | Frame profiler |
| Telemetry overhead | <1ms | Event timing |
| Total latency (cache HIT) | <50ms | End-to-end |

---

**Status:** Ready to begin implementation  
**Next Step:** Create Widget-Log Python module interface
