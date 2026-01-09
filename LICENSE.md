# OptaFly_Zed License

## TL;DR (Quick Summary)

| Usage Scenario | License | Source Disclosure Required? |
|----------------|---------|----------------------------|
| **Full OptaFly_Zed editor** (personal/internal use) | AGPL-3.0-or-later | ❌ No (unless networked) |
| **Full OptaFly_Zed editor** (network/SaaS deployment) | AGPL-3.0-or-later | ✅ Yes (AGPL copyleft) |
| **Widget-Log only** (standalone library) | MIT OR Apache-2.0 | ❌ No (permissive) |
| **OptaCore only** (standalone library) | MIT OR Apache-2.0 | ❌ No (permissive) |

---

## Overview

**OptaFly_Zed** is a derivative work (fork + enhancements) of the [Zed editor](https://github.com/zed-industries/zed). As a combined work, it inherits the upstream copyleft requirements while adding permissively-licensed components.

### Key Principle (Copyleft Inheritance):

Since OptaFly_Zed builds upon AGPL-3.0 code from upstream Zed, **the entire combined distribution must be offered under AGPL-3.0-or-later** when distributing binaries or modified source of the full editor.

However, **OptaFly-specific additions** (Widget-Log, OptaCore) are dual-licensed MIT/Apache-2.0 and can be extracted and used independently under those permissive terms.

---

## 1. Upstream Zed Editor (AGPL-3.0 / GPL-3.0)

**Majority of this codebase** comes from [Zed Industries](https://github.com/zed-industries/zed):

**Copyright 2022 - 2025 Zed Industries, Inc.**

### Licensing by Component:

| Component | License | File Reference |
|-----------|---------|----------------|
| **Editor/Client Code** | GPL-3.0 or AGPL-3.0-or-later | [LICENSE-GPL](LICENSE-GPL), [LICENSE-AGPL](LICENSE-AGPL) |
| **Server-Side Components** | AGPL-3.0 | [LICENSE-AGPL](LICENSE-AGPL) |
| **GPUI Framework** | Apache-2.0 | [LICENSE-APACHE](LICENSE-APACHE) |

**Primary License for Full Editor: AGPL-3.0-or-later**

### Why AGPL-3.0?

Zed uses copyleft licensing to:
- Ensure community benefits from improvements
- Prevent proprietary closed-source derivatives
- Allow commercial services **on top of** Zed while requiring source disclosure for modifications

### AGPL-3.0 Key Obligations:

✅ **You CAN:**
- Use OptaFly_Zed internally within your organization
- Modify and redistribute with source code
- Offer commercial services using OptaFly_Zed
- Charge for support, hosting, or custom deployments

⚠️ **You MUST:**
- Provide **complete source code** to users if you deploy over a network (SaaS, web app, remote access)
- Include license notices and copyright
- License derivative works under AGPL-3.0-or-later
- Clearly indicate any modifications you made

❌ **You CANNOT:**
- Create proprietary closed-source derivatives of the full editor
- Remove or hide source code disclosure requirements
- Relicense AGPL code under permissive terms (MIT/BSD/Apache)

### Affected Components:

```
crates/editor/         — Core editor logic
crates/workspace/      — Workspace management
crates/project/        — Project/file tree
crates/gpui/           — GPU-accelerated UI framework (Apache-2.0 upstream, AGPL in combined work)
crates/lsp/            — Language Server Protocol
crates/language/       — Language support
crates/collab/         — Collaboration server (AGPL-3.0)
crates/rpc/            — RPC communication
crates/vim/            — Vim mode
crates/ui/             — UI components
crates/theme/          — Theme system
crates/zed/            — Main binary entry point
```

---

## 2. OptaFly Enhancements (MIT/Apache-2.0 Dual License)

**OptaFly-specific additions** are independently licensed under **your choice** of:

- **MIT License** (see [full text below](#mit-license-full-text))
- **Apache License 2.0** (see [LICENSE-APACHE](LICENSE-APACHE))

**Copyright (c) 2025 Tumquan Corp**

### What This Means:

- If you extract **only** these components (as standalone libraries), you can use them under MIT or Apache-2.0
- When combined with Zed editor, the **overall work** is AGPL-3.0 (copyleft wins)
- Permissive → AGPL is allowed; AGPL → Permissive is **not**

### OptaFly-Specific Components:

#### 1. Widget-Log Semantic Caching Integration
```
widget-log/                              — Full semantic caching system
  ├── secure_proxy.py                    — HTTPS proxy with authentication
  ├── cache_manager.py                   — Semantic similarity engine
  ├── embedding_service.py               — 384-dim sentence transformers
  ├── start-proxy.sh                     — Startup script
  └── [all other Widget-Log files]
```

**Features:**
- 280x faster AI responses on cache hits
- 95% semantic similarity matching
- FAISS vector search
- Secure localhost-only HTTPS
- Multi-project intelligence

#### 2. OptaCore Architecture Engine
```
crates/optacore_struct/                  — Tensor-based architecture modeling
  ├── src/optacore_struct.rs            — Force-directed layout (Fruchterman-Reingold)
  ├── src/anti_patterns.rs              — Cycle/bottleneck detection
  └── src/visualization.rs              — C4-compliant DOT export

crates/optacore_jni/                     — Structurizr JNI bridge
  ├── src/optacore_jni.rs               — Java Native Interface
  ├── README.md                          — Integration guide
  └── QUICKSTART.md                      — Getting started
```

**Features:**
- Automatic diagram optimization
- Anti-pattern detection (cycles, bottlenecks, over-coupling)
- GPU acceleration option (WGPU)
- Telemetry for ML training
- C4 diagram export

#### 3. Build & Installation Scripts
```
install-phase25-parallel.sh              — Parallel dependency installer
PHASE25_PARALLEL_INSTALL.md             — Installation documentation
[OptaFly-specific build configs]
```

#### 4. Documentation
```
WIDGET_LOG_INTEGRATION.md               — Widget-Log integration guide
WIDGET_LOG_NATIVE_INTEGRATION_PLAN.md   — Technical architecture
BUILD_INSTRUCTIONS.md                    — Build guide
[OptaFly-specific Phase 2.5 docs]
```

### Choosing Your License (for standalone use):

| License | Best For | Key Benefits |
|---------|----------|--------------|
| **MIT** | Simplicity, maximum freedom | Short, permissive, allows proprietary derivatives |
| **Apache-2.0** | Patent protection, enterprise | Explicit patent grant, contributor protections |

---

## 3. Combined Work License (The Important Part!)

### When You Use the Full OptaFly_Zed Editor:

**License: AGPL-3.0-or-later applies to the entire combined work**

This is because:
1. Zed editor base is AGPL-3.0
2. Copyleft licenses "infect" derivative works
3. OptaFly additions (permissive) are **compatible** with AGPL but become part of the AGPL work

### Practical Scenarios:

#### ✅ Scenario 1: Internal Corporate Use
```
Company XYZ uses OptaFly_Zed for 200 developers
License: AGPL-3.0-or-later
Source disclosure required? NO (internal use only)
```

#### ⚠️ Scenario 2: SaaS/Remote Development Platform
```
Company ABC offers "OptaFly Cloud" — web-based IDE
License: AGPL-3.0-or-later
Source disclosure required? YES (network deployment)
Must provide: Complete source code to all users
```

#### ✅ Scenario 3: Using Only Widget-Log
```
Company DEF integrates Widget-Log into proprietary product
License: MIT or Apache-2.0 (your choice)
Source disclosure required? NO (permissive license)
Can be closed-source? YES
```

#### ✅ Scenario 4: Using Only OptaCore
```
Startup builds commercial architecture tool with OptaCore
License: MIT or Apache-2.0 (your choice)
Source disclosure required? NO
Can sell proprietary licenses? YES
```

---

## 4. Commercial Use Guidance

### For Enterprises Considering OptaFly_Zed:

**Good News:**
- No "commercial license" purchase required
- Internal use is unrestricted (no source disclosure)
- Can build commercial services on top

**Important Considerations:**
- **Network deployment triggers AGPL**: If employees/customers access remotely, source disclosure required
- **Not "open core"**: No dual-licensing available for the full editor
- **Consult legal counsel**: AGPL interpretation varies by jurisdiction

### Alternative Approaches:

| Approach | License Impact | Complexity |
|----------|----------------|------------|
| Use full OptaFly_Zed internally | AGPL, no disclosure | Low |
| Extract Widget-Log + OptaCore only | MIT/Apache-2.0 | Medium (rebuild integration) |
| Fork and upstream contributions | AGPL, community benefits | High (CLA required) |

---

## 5. Contributing to OptaFly_Zed

### Contributor License Agreement (CLA):

By submitting contributions, you agree that:

#### For Contributions to Zed Base Code:
- Your changes are licensed under **AGPL-3.0-or-later** (same as upstream)
- For upstream Zed: Must sign [Zed CLA](https://zed.dev/cla)

#### For Contributions to OptaFly Enhancements:
- Your changes are dual-licensed **MIT AND Apache-2.0**
- Allows maximum reuse and compatibility

#### General Terms:
- You have the legal right to submit the contribution
- You understand the licensing implications
- You agree to the project's [Code of Conduct](https://zed.dev/code-of-conduct)

### Upstream Contributions:

To contribute changes back to Zed:
1. Sign the [Zed CLA](https://zed.dev/cla)
2. Follow [Zed contribution guidelines](https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md)
3. Submit PR to [zed-industries/zed](https://github.com/zed-industries/zed)

---

## 6. Third-Party Dependencies

This project includes numerous third-party dependencies, each with their own licenses:

### Icon and Theme Licenses:
- **Icons**: See [assets/icons/LICENSES](assets/icons/LICENSES)
- **Themes**: See [assets/themes/LICENSES](assets/themes/LICENSES)

### Rust Dependencies:
Check individual crates via `Cargo.toml` and `Cargo.lock`:
```bash
cargo tree --prefix none | grep -v "^  " | sort -u
```

### Python Dependencies (Widget-Log):
```
widget-log/venv/lib/python3.12/site-packages/*/LICENSE*
```

All dependencies are compatible with AGPL-3.0 distribution.

---

## 7. License Compatibility Matrix

| Incoming License | Can Combine with AGPL? | Outgoing License |
|------------------|------------------------|------------------|
| MIT | ✅ Yes | AGPL-3.0 |
| Apache-2.0 | ✅ Yes | AGPL-3.0 |
| BSD-3-Clause | ✅ Yes | AGPL-3.0 |
| GPL-3.0 | ✅ Yes | AGPL-3.0 |
| LGPL-3.0 | ✅ Yes | AGPL-3.0 |
| Proprietary | ❌ No | N/A |

**Key Rule**: Permissive licenses can be "upgraded" to copyleft. Copyleft cannot be "downgraded" to permissive.

---

## 8. MIT License (Full Text)

For OptaFly-specific components when used standalone:

```
MIT License

Copyright (c) 2025 Tumquan Corp

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 9. Additional License Files

Full license texts are available in:

- **[LICENSE-AGPL](LICENSE-AGPL)** — AGPL-3.0 (primary for full editor)
- **[LICENSE-APACHE](LICENSE-APACHE)** — Apache-2.0 (GPUI framework, OptaFly components option)
- **[LICENSE-GPL](LICENSE-GPL)** — GPL-3.0 (older Zed client code, compatibility)

---

## 10. Questions and Support

### Licensing Questions:

**For Zed Editor Base:**
- 📧 Contact: [Zed Industries](https://zed.dev/docs/community)
- 🐙 Upstream: [zed-industries/zed](https://github.com/zed-industries/zed)
- 📜 Zed License Info: [Zed Licensing](https://zed.dev/docs/community#licensing)

**For OptaFly Enhancements:**
- 🐙 Repository: [Optaquan/OptaFly_Zed](https://github.com/Optaquan/OptaFly_Zed)
- 🐛 Issues: [OptaFly_Zed Issues](https://github.com/Optaquan/OptaFly_Zed/issues)
- 📧 Contact: See repository for contact information

**For Commercial/Legal Advice:**
- Consult with legal counsel experienced in AGPL-3.0, MIT, and Apache-2.0
- Review FSF's [AGPL FAQ](https://www.gnu.org/licenses/agpl-3.0.html)
- Consider [OSI's License Review](https://opensource.org/licenses)

### Reporting License Issues:

If you discover licensing inconsistencies:
1. Open an issue with clear description
2. Reference specific files/components
3. Provide suggested corrections

---

## Acknowledgments

**OptaFly_Zed** builds on the outstanding work of:

- **[Zed Industries](https://zed.dev)** — Creating the blazing-fast Zed editor
- **Upstream Contributors** — Thousands of commits from the open-source community
- **AGPL Philosophy** — Ensuring software freedom and community collaboration

### About Optaquan and the OptaFly Development Teams

**OptaFly_Zed** enhancements are developed by:

- **Optaquan** — A performance optimization brand of **Tumquan Corp**
- **OptaFly Development Teams** — Engineering teams focused on:
  - Widget-Log semantic caching integration
  - OptaCore tensor-based architecture modeling
  - Performance optimizations and ML foundations
  - Developer experience improvements

**Tumquan Corp** (Copyright holder) is committed to advancing developer tools through intelligent caching, architectural optimization, and open-source collaboration.

### Our Commitments:

- ✅ Honoring upstream licenses and respecting original creators
- ✅ Contributing improvements back to the Zed community
- ✅ Maintaining transparency in licensing and development practices
- ✅ Supporting the open-source ecosystem
- ✅ Building performance-first developer tools
- ✅ Fostering collaboration between commercial and open-source interests

---

## Legal Disclaimer

**This LICENSE.md provides guidance and summary. The actual license texts (LICENSE-AGPL, LICENSE-APACHE, LICENSE-GPL) are the authoritative legal documents.**

In case of conflict between this summary and the full license texts, the full texts take precedence. This document does not constitute legal advice.

**For legally binding terms, refer to:**
- [LICENSE-AGPL](LICENSE-AGPL) for AGPL-3.0 terms
- [LICENSE-APACHE](LICENSE-APACHE) for Apache-2.0 terms  
- [LICENSE-GPL](LICENSE-GPL) for GPL-3.0 terms

---

**Last Updated**: January 9, 2026  
**OptaFly_Zed Version**: Phase 2.5  
**License Document Version**: 1.0
