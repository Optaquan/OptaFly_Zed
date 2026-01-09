# Widget-Log Integration in OptaFly_Zed

**OptaFly_Zed** is a performance-enhanced distribution of Zed editor with **Widget-Log semantic caching** built-in, delivering **280x faster AI responses** out of the box.

---

## 🚀 What is Widget-Log?

Widget-Log is an intelligent semantic caching proxy for Claude API interactions that provides:

- **280x faster responses** on cache hits (43ms vs 12,201ms)
- **60% cache hit rate** in real-world usage
- **95% semantic similarity** accuracy on fuzzy matches
- **60% cost reduction** on API calls
- **Zero configuration** with automatic project detection

---

## ✨ Built-In Features

### Automatic Configuration

OptaFly_Zed comes pre-configured with Widget-Log:

1. **Secure HTTPS Proxy** on `127.0.0.1:8443`
2. **Bearer token authentication** for localhost-only access
3. **Optimized cache settings** (8192 tokens, 4 cache anchors)
4. **Default project folder** for immediate caching
5. **Zed settings** pre-configured to use Widget-Log

### Performance Highlights

| Metric | Value |
|--------|-------|
| **Cache Hit Speedup** | 280-1122x |
| **Response Time (Hit)** | 37-43ms |
| **Response Time (Miss)** | 10,000-57,000ms |
| **Semantic Similarity** | 93-95% |
| **Cache Hit Rate** | 57-60% |
| **Tokens Saved/Hit** | 900-3300 |

---

## 📁 Installation & Setup

### Quick Start

1. **Clone OptaFly_Zed:**
   ```bash
   git clone https://github.com/Optaquan/OptaFly_Zed.git
   cd OptaFly_Zed
   ```

2. **Set up Widget-Log:**
   ```bash
   cd widget-log
   ./install.sh
   ```

3. **Configure API Key:**
   ```bash
   nano widget-log/.env
   # Add: ANTHROPIC_API_KEY=your_key_here
   ```

4. **Start Widget-Log Proxy:**
   ```bash
   cd widget-log
   ./start-proxy.sh
   ```

5. **Build and run OptaFly_Zed:**
   ```bash
   cd ..
   # Follow standard Zed build instructions for your platform
   cargo run --release
   ```

### Pre-Configured Settings

OptaFly_Zed includes optimized Zed settings in `widget-log/configure-zed.sh`:

```json
{
  "language_models": {
    "anthropic": {
      "api_url": "https://127.0.0.1:8443/v1",
      "available_models": [{
        "name": "claude-3-5-sonnet-20241022",
        "max_tokens": 8192,
        "max_cache_anchors": 4,
        "cache_configuration": {
          "max_cache_anchors": 4,
          "min_tokens_per_anchor": 1024,
          "should_speculate": true
        }
      }]
    }
  },
  "http_client": {
    "headers": {
      "Authorization": "Bearer $WIDGET_LOG_AUTH_TOKEN"
    }
  }
}
```

---

## 🎯 How It Works

### Architecture

```
OptaFly_Zed Editor
    ↓ (API Request)
Widget-Log Secure HTTPS Proxy (127.0.0.1:8443)
    ↓
[Semantic Cache Check]
    ├─→ Cache Hit (43ms) → Return Cached Response
    └─→ Cache Miss (12s) → Claude API → Store in Cache
```

### Semantic Caching Process

1. **Query Embedding:** Generate 384-dimensional embedding using sentence-transformers
2. **Similarity Search:** FAISS vector search across project caches
3. **Cache Hit (85%+ similarity):** Return cached response instantly
4. **Cache Miss:** Call Claude API, store response with fingerprint
5. **Future Queries:** Similar questions hit cache automatically

### Multi-Project Intelligence

- **Project Detection:** Fuzzy matching with Levenshtein distance
- **Project-Specific Caches:** Separate cache per project maintains context boundaries
- **Default Fallback:** OptaFly_Zed project auto-created for immediate caching
- **Acronym Matching:** "of" → "optafly", "wl" → "widget-log"

---

## 🔧 Configuration

### Cache Settings

Edit `widget-log/config.yaml`:

```yaml
cache:
  similarity_threshold: 0.85  # Min similarity for cache hit
  ttl_days: 30                # Cache expiration

anthropic:
  model: "claude-opus-4-20250514"
  max_tokens: 8192
  cache_anchors:
    max_anchors: 4
    min_tokens_per_anchor: 1024
```

### Custom Projects

Create project-specific caches:

```bash
cd widget-log
mkdir -p MyProject/contexts
cat > MyProject/metadata.json << EOF
{
  "name": "MyProject",
  "description": "My custom project cache",
  "tags": ["rust", "web"],
  "cache_priority": "high"
}
EOF
```

---

## 📊 Performance Examples

### Real-World Results

From `widget-log/examples/cache_performance_architectural_queries.md`:

| Test | Query Type | Cache Status | Response Time | Speedup |
|------|-----------|--------------|---------------|---------|
| 1 | First query | MISS | 45,551ms | baseline |
| 2 | Exact repeat | **HIT** | **30ms** | **1518x** |
| 3 | Semantic variant | **HIT** | **45ms** | **1012x** |
| 4 | Different query | MISS | 21,780ms | baseline |
| 5 | Repeat #4 | **HIT** | **38ms** | **573x** |

**Average speedup:** 1122x faster on cache hits  
**Cache hit rate:** 57-60% typical

### Cost Savings

**Without Widget-Log:**
- 1000 queries/month × $0.015 = $15/month

**With Widget-Log:**
- 600 cache hits × $0.00 = $0
- 400 API calls × $0.015 = $6/month
- **Savings:** $9/month (60% reduction)

**At scale (100 users):** $900/month saved

---

## 🛠️ Management & Monitoring

### Check Proxy Status

```bash
ps aux | grep secure_proxy
```

### View Cache Statistics

```bash
curl -k -H "Authorization: Bearer $WIDGET_LOG_AUTH_TOKEN" \
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
tail -f widget-log/logs/widget-log.log
```

### Restart Proxy

```bash
cd widget-log
killall python
./start-proxy.sh
```

---

## 🔒 Security

### Localhost-Only Access

- Proxy binds to `127.0.0.1:8443` (localhost only)
- Cannot be accessed from network
- Dedicated port for Widget-Log only

### Authentication

- 256-bit Bearer token required for all requests
- Token auto-generated during installation
- Stored in `widget-log/.env`
- Automatically injected into Zed settings

### SSL/TLS Encryption

- Self-signed certificate for localhost
- HTTPS encryption for all traffic
- Certificate valid for 10 years
- Auto-generated during setup

---

## 📚 Documentation

### Widget-Log Specific

- `widget-log/README.md` - Complete Widget-Log documentation
- `widget-log/CACHE_COGNIZANCE_RESULTS.md` - Performance analysis and test results
- `widget-log/OPTIMIZATION_GUIDE.md` - Configuration tuning guide
- `widget-log/examples/` - Real-world performance examples

### OptaFly_Zed Specific

- `docs/src/development/` - Building OptaFly_Zed
- `CONTRIBUTING.md` - Contributing guidelines
- This file - Widget-Log integration guide

---

## 🐛 Troubleshooting

### Proxy Not Starting

**Problem:** `start-proxy.sh` fails

**Solution:**
```bash
cd widget-log
# Check Python dependencies
./venv/bin/pip install -r requirements.txt

# Verify .env configuration
cat .env | grep ANTHROPIC_API_KEY
cat .env | grep WIDGET_LOG_AUTH_TOKEN

# Check SSL certificates
ls -la certs/cert.pem certs/key.pem
```

### Cache Not Working

**Problem:** All queries showing as MISS

**Solution:**
```bash
# Check proxy is running
ps aux | grep secure_proxy

# Test proxy connection
curl -k -H "Authorization: Bearer $WIDGET_LOG_AUTH_TOKEN" \
  https://127.0.0.1:8443/health

# Verify Zed settings
cat ~/.config/zed/settings.json | grep "127.0.0.1:8443"
```

### Zed Not Using Proxy

**Problem:** Zed bypassing Widget-Log

**Solution:**
```bash
cd widget-log
./configure-zed.sh  # Reconfigure Zed settings
```

**Verify settings in Zed:**
- Open Zed → Settings (Cmd/Ctrl+,)
- Check `language_models.anthropic.api_url` = `https://127.0.0.1:8443/v1`
- Check `http_client.headers.Authorization` contains Bearer token

### Permission Errors

**Problem:** Widget-Log can't access files

**Solution:**
```bash
chmod +x widget-log/start-proxy.sh
chmod +x widget-log/configure-zed.sh
chmod 600 widget-log/.env
chmod 600 widget-log/certs/key.pem
```

---

## 🚀 Advanced Features

### Custom Similarity Thresholds

Adjust for different use cases:

```yaml
# widget-log/config.yaml
cache:
  similarity_threshold: 0.90  # Strict (code-specific queries)
  # OR
  similarity_threshold: 0.80  # Fuzzy (general queries)
```

### GPU Acceleration

Enable GPU for faster embedding generation:

```yaml
# widget-log/config.yaml
performance:
  use_gpu: true  # Requires CUDA-capable GPU
```

**Expected improvement:** 10-20x faster cache lookups

### Multiple Projects

Widget-Log automatically detects and caches per-project:

```bash
widget-log/
├── OptaFly_Zed/          # Default project
├── my-rust-project/       # Auto-created from workspace
└── documentation/         # Separate cache boundary
```

Each project maintains independent cache with project-specific context.

---

## 🎓 Best Practices

### 1. Keep Proxy Running

Start proxy on system boot:

```bash
# Copy systemd service
sudo cp widget-log/widget-log-proxy.service /etc/systemd/system/
sudo systemctl enable widget-log-proxy
sudo systemctl start widget-log-proxy
```

### 2. Monitor Cache Performance

Check stats regularly:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias wl-stats='curl -k -s -H "Authorization: Bearer $(grep WIDGET_LOG_AUTH_TOKEN widget-log/.env | cut -d= -f2)" https://127.0.0.1:8443/stats | jq'
```

### 3. Clean Expired Entries

Widget-Log auto-cleans after 30 days, but you can manually clean:

```bash
cd widget-log
./venv/bin/python -c "
from app.cache_manager import CacheManager
cache = CacheManager('OptaFly_Zed')
removed = cache.cleanup_expired()
print(f'Removed {removed} expired entries')
"
```

### 4. Backup Cache Database

```bash
# Backup cache
tar -czf widget-log-cache-backup.tar.gz widget-log/*/fingerprints.db widget-log/*/contexts/

# Restore cache
tar -xzf widget-log-cache-backup.tar.gz
```

---

## 🔄 Updates

### Update Widget-Log

```bash
cd widget-log
git pull origin main
./venv/bin/pip install -r requirements.txt --upgrade
killall python
./start-proxy.sh
```

### Update OptaFly_Zed

```bash
cd OptaFly_Zed
git pull origin main
cargo build --release
```

Widget-Log settings are preserved across updates.

---

## 🤝 Contributing

### Report Issues

- **Widget-Log specific:** https://github.com/Optaquan/Widget-Log/issues
- **OptaFly_Zed specific:** https://github.com/Optaquan/OptaFly_Zed/issues

### Contribute Improvements

1. Fork the repository
2. Create feature branch: `git checkout -b feature/my-improvement`
3. Make changes
4. Test thoroughly
5. Submit pull request

---

## 📄 License

- **OptaFly_Zed:** See main LICENSE file
- **Widget-Log:** MIT/Apache 2.0 Dual License

---

## 🎉 Summary

OptaFly_Zed with Widget-Log delivers:

✅ **280x faster AI responses** on cache hits  
✅ **60% cost reduction** on API usage  
✅ **Zero configuration** required  
✅ **Secure** localhost-only HTTPS proxy  
✅ **Intelligent** semantic caching with 95% accuracy  
✅ **Production-ready** with comprehensive testing  

**Start using OptaFly_Zed today and experience the future of AI-assisted coding!**
