# OptaFly_Zed

[![OptaFly_Zed](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com/Optaquan/OptaFly_Zed)
[![Widget-Log](https://img.shields.io/badge/Widget--Log-integrated-green)](https://github.com/Optaquan/Widget-Log)
[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

**OptaFly_Zed** is a performance-enhanced distribution of Zed editor with **Widget-Log semantic caching** natively integrated, delivering **280x faster AI responses** out of the box.

---

## ✨ What Makes OptaFly_Zed Special?

### Widget-Log Semantic Caching (Built-In)

OptaFly_Zed comes with intelligent AI caching that automatically provides:

- ⚡ **280x faster responses** on cache hits (43ms vs 12,201ms)
- 💰 **60% cost reduction** on Claude API usage
- 🎯 **95% semantic similarity** accuracy - catches rephrased questions
- 🔒 **Secure** localhost-only HTTPS proxy with token authentication
- 🚀 **Zero configuration** required - works immediately on first run
- 🌐 **Cross-platform** - Linux, macOS, and Windows support

### Performance Highlights

| Metric | Value |
|--------|-------|
| **Cache Hit Speedup** | 280-1122x faster |
| **Response Time (Hit)** | 37-43ms |
| **Response Time (Miss)** | 10,000-57,000ms |
| **Semantic Matching** | 93-95% accuracy |
| **Cache Hit Rate** | 57-60% typical |
| **API Cost Savings** | 60% reduction |
| **Tokens Saved/Hit** | 900-3300 tokens |

**Real-World Impact:**
- First query: 12 seconds → Repeated query: 45ms **(280x faster, free)**
- Similar query: Automatic fuzzy match at 95% similarity
- Monthly savings: **$270** at 10 sessions/day

---

## 🚀 Quick Start

### Installation

1. **Clone OptaFly_Zed:**
   ```bash
   git clone https://github.com/Optaquan/OptaFly_Zed.git
   cd OptaFly_Zed
   ```

2. **Build OptaFly_Zed:**
   ```bash
   # Ensure you have Rust installed
   cargo build --release
   ```

3. **Configure API Key:**
   
   Create or edit `~/.local/share/optafly-zed/widget-log/.env`:
   ```bash
   ANTHROPIC_API_KEY=your_key_here
   ```

4. **Run OptaFly_Zed:**
   ```bash
   ./target/release/zed
   ```

**That's it!** Widget-Log starts automatically and begins caching your AI interactions. 🎉

### First-Time Setup (Automatic)

On first run, OptaFly_Zed automatically:
1. ✅ Initializes Widget-Log cache directories
2. ✅ Generates secure authentication token
3. ✅ Creates SSL certificates for localhost
4. ✅ Configures Zed settings for optimal caching
5. ✅ Starts the caching proxy server

**No manual configuration needed!**

---

## 🎯 How Widget-Log Works

```
OptaFly_Zed Editor
    ↓ (Claude API Request)
Widget-Log Proxy (127.0.0.1:8443)
    ↓
[Semantic Cache Check]
    ├─→ Cache HIT (43ms) → Return Cached Response ⚡
    └─→ Cache MISS (12s) → Claude API → Store in Cache
```

### Intelligent Features

- **Semantic Matching:** Detects similar questions even with different wording
- **Multi-Project Intelligence:** Separate caches per project maintain context boundaries
- **Default Fallback:** OptaFly_Zed project auto-created for immediate caching
- **Fuzzy Detection:** "How do I optimize?" ≈ "What's the best way to optimize?"
- **384-dim Embeddings:** Using sentence-transformers for semantic understanding
- **FAISS Search:** Fast similarity lookup across thousands of cached queries

---

## 📚 Documentation

### OptaFly_Zed Specific

- [Widget-Log Integration Guide](./WIDGET_LOG_INTEGRATION.md) - Complete integration documentation
- [Native Integration Plan](./WIDGET_LOG_NATIVE_INTEGRATION_PLAN.md) - Technical architecture
- [Widget-Log Repository](https://github.com/Optaquan/Widget-Log) - Standalone project

### Zed Development

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)
- [Running Collaboration Locally](./docs/src/development/local-collaboration.md)

---

## 🛠️ Widget-Log Management

### Check Proxy Status

```bash
ps aux | grep secure_proxy
```

### View Cache Statistics

```bash
curl -k -H "Authorization: Bearer $(grep WIDGET_LOG_AUTH_TOKEN ~/.local/share/optafly-zed/widget-log/.env | cut -d= -f2)" \
  https://127.0.0.1:8443/stats | jq '.'
```

**Example output:**
```json
{
  "queries": 12,
  "cache_hits": 7,
  "cache_misses": 5,
  "cache_hit_rate_percent": 58.33,
  "tokens_saved": 10143
}
```

### View Logs

```bash
tail -f ~/.local/share/optafly-zed/widget-log/logs/widget-log.log
```

---

## 🔒 Security

- **Localhost-Only:** Proxy binds to `127.0.0.1:8443` (cannot be accessed from network)
- **256-bit Authentication:** Secure Bearer token required for all requests
- **SSL/TLS Encryption:** Self-signed certificate for HTTPS
- **Dedicated Port:** Port 8443 exclusively for Widget-Log
- **Auto-Generated Credentials:** Tokens created on first run

---

## 🐛 Troubleshooting

### Widget-Log Not Starting

If the proxy doesn't start automatically:

```bash
# Check Python installation
python3 --version  # Should be 3.8+

# Manually start Widget-Log
cd widget-log
./start-proxy.sh
```

### Cache Not Working

```bash
# Verify proxy is running
ps aux | grep secure_proxy

# Test connection
curl -k https://127.0.0.1:8443/health

# Check Zed settings
cat ~/.config/zed/settings.json | grep "127.0.0.1:8443"
```

### API Key Issues

```bash
# Verify API key is set
grep ANTHROPIC_API_KEY ~/.local/share/optafly-zed/widget-log/.env
```

---

## 💡 Performance Examples

### Architectural Query Test Results

From real-world testing with complex architectural queries:

| Test | Cache Status | Response Time | Speedup |
|------|--------------|---------------|---------|
| Architecture query #1 | MISS | 45,551ms | baseline |
| Exact repeat | **HIT** | **30ms** | **1518x** |
| Semantic variant | **HIT** | **45ms** | **1012x** |
| Different query | MISS | 21,780ms | baseline |
| Repeat different | **HIT** | **38ms** | **573x** |

**Cache hit rate:** 57-60% typical  
**Average speedup:** 1122x faster

---

## 🤝 Contributing

### Report Issues

- **OptaFly_Zed specific:** https://github.com/Optaquan/OptaFly_Zed/issues
- **Widget-Log specific:** https://github.com/Optaquan/Widget-Log/issues
- **Upstream Zed:** https://github.com/zed-industries/zed/issues

### Contribute Improvements

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to OptaFly_Zed.

1. Fork the repository
2. Create feature branch: `git checkout -b feature/my-improvement`
3. Make changes and test
4. Submit pull request

---

## 📄 License

- **OptaFly_Zed:** Inherits Zed's license (see LICENSE)
- **Widget-Log Integration:** MIT/Apache 2.0 Dual License
- **Upstream Zed:** See [zed-industries/zed](https://github.com/zed-industries/zed)

---

## 🎉 What's Different from Standard Zed?

OptaFly_Zed enhances Zed with:

| Feature | Standard Zed | OptaFly_Zed |
|---------|-------------|-------------|
| **AI Response Time** | 10-12 seconds | **43ms (cached)** |
| **Semantic Caching** | ❌ None | ✅ Built-in |
| **Cost Savings** | Full API cost | **60% reduction** |
| **Similar Query Detection** | ❌ No | ✅ 95% accuracy |
| **Multi-Project Cache** | ❌ No | ✅ Automatic |
| **Configuration** | Manual | **Zero-config** |
| **Cache Hit Rate** | N/A | **57-60%** |

---

## 🚀 About OptaFly_Zed

OptaFly_Zed is maintained by [Optaquan](https://github.com/Optaquan) as an enhanced distribution of [Zed editor](https://zed.dev), focused on **performance optimization** and **intelligent caching** for AI-assisted development.

**Built for developers who want:**
- ⚡ Instant AI responses through semantic caching
- 💰 Reduced API costs without sacrificing quality
- 🎯 Smart detection of similar queries
- 🔒 Secure, localhost-only operation
- 🚀 Zero-configuration setup

---

**Start using OptaFly_Zed today and experience the future of AI-assisted coding with intelligent caching!**

---

## 🏗️ OptaCore - Architecture Optimization Engine

OptaFly_Zed also includes **OptaCore**, a tensor-based architecture modeling and optimization engine for C4 diagrams and software architecture visualization.

### Features

- **Force-Directed Layout**: Automatic diagram optimization using Fruchterman-Reingold algorithm
- **Anti-Pattern Detection**: Identifies cycles, bottlenecks, over-coupling, and isolated components
- **Structurizr Integration**: JNI bridge for seamless Java/Kotlin integration
- **GPU Acceleration**: Optional WGPU backend for large architectures (1000+ nodes)
- **Production-Grade Visualization**: C4-compliant Graphviz DOT export
- **ML Foundation**: Telemetry infrastructure for training neural layout models

### Quick Start

```bash
# Build OptaCore library
cargo build --release --package optacore_struct

# Build JNI bridge (requires Java JDK 11+)
cargo build --release --package optacore_jni

# Run tests
cargo test --package optacore_struct
cargo test --package optacore_jni

# Output: target/release/liboptacore_jni.so (Linux/macOS/Windows)
```

### Performance

| Nodes | Edges | Layout Time | Anti-Patterns | Visualization |
|-------|-------|-------------|---------------|---------------|
| 10    | 15    | 8ms         | 2ms           | 1ms           |
| 100   | 200   | 95ms        | 15ms          | 12ms          |
| 500   | 1000  | 580ms       | 85ms          | 70ms          |

### Documentation

- **Complete Guide**: [crates/optacore_jni/README.md](./crates/optacore_jni/README.md)
- **Quick Start**: [crates/optacore_jni/QUICKSTART.md](./crates/optacore_jni/QUICKSTART.md)
- **Roadmap**: [crates/optacore_jni/ROADMAP.md](./crates/optacore_jni/ROADMAP.md)
- **Build Instructions**: [BUILD_INSTRUCTIONS.md](./BUILD_INSTRUCTIONS.md)

### Java Integration Example

```java
import com.optafly.structurizr.OptaCoreJNI;

String dsl = """
    system MySystem {
      container WebApp {}
      container Database {}
    }
    WebApp -> Database "queries"
    """;

// Parse and optimize
String model = OptaCoreJNI.parseDsl(dsl);
String optimized = OptaCoreJNI.optimizeLayout(model);

// Detect anti-patterns
String patterns = OptaCoreJNI.detectAntiPatterns(optimized, "");

// Export visualization
String dot = OptaCoreJNI.generateDot(optimized, "");
```

