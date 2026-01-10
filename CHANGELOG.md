# Changelog

All notable changes to OptaFly_Zed will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-01-09 - Stable Release 🎉

### 🚀 Major Milestone

This is the first **stable release** of OptaFly_Zed, marking the completion of Phase 2.5 and production readiness for all core features.

### Added

#### Installation & Distribution
- **Cross-Platform Installers**: One-click installation scripts for all platforms
  - `install.sh` - Automated Linux installer with distro detection
  - `install-macos.sh` - macOS installer with Homebrew integration
  - `install-windows.ps1` - PowerShell installer with Chocolatey support
- **Comprehensive INSTALL.md**: 500+ line installation guide with:
  - Platform-specific prerequisites and dependencies
  - Step-by-step manual installation instructions
  - Troubleshooting guide for common issues
  - Upgrade paths from previous versions
  - Verification and testing procedures

#### Version Cohesion
- **Pinned Rust Toolchain**: Locked to Rust 1.91.1 in `rust-toolchain.toml`
- **Pinned Python Dependencies**: Exact versions in `widget-log/requirements.txt`
  - anthropic==0.40.0
  - sentence-transformers==3.1.1
  - faiss-cpu==1.9.0
  - aiohttp==3.11.10
  - All dependencies pinned for stability
- **Workspace Version**: Added version = "1.0.0" to Cargo.toml workspace
- **Zed Crate Version**: Updated from 0.217.4 to 1.0.0

#### Documentation
- **Version Badges**: Updated README.md with v1.0.0 badges
- **Installation Instructions**: Prominent v1.0.0 quick start section
- **Feature Highlights**: Comprehensive feature comparison table

### Changed

- **Installation Scripts**: Renamed to version-specific format
  - `install-phase25-parallel.sh` → `install.sh`
  - `install-mac.sh` → `install-macos.sh`
  - `install-windows.ps1` → `install-windows.ps1`
- **README.md**: Restructured with v1.0.0 installation as primary method
- **Version Strategy**: Transitioned from phase-based (0.9x) to semantic versioning (1.x.x)

### Fixed

- **Dependency Conflicts**: Pinned versions prevent version mismatch issues
- **Build Reproducibility**: Locked Rust toolchain ensures consistent builds
- **Cross-Platform Compatibility**: Installers tested on Ubuntu, Fedora, Arch, macOS, Windows

### Technical Details

**Version Numbers:**
- Workspace: 1.0.0
- Zed Crate: 1.0.0 (was 0.217.4)
- Rust Toolchain: 1.91.1 (pinned)
- Python Requirement: 3.12+ (pinned packages)

**Release Artifacts:**
- Source code (tar.gz, zip)
- Installation scripts (Linux, macOS, Windows)
- Documentation (INSTALL.md, README.md)

---

## [0.99.0] - 2026-01-09 (Phase 2.5 Release)

### Added
- **Complete Phase 2.5**: ML Foundation + Structurizr JNI Integration
- **OptaCore JNI Bridge**: Native Java integration for Structurizr workspace analysis
- **C4 DSL Parser**: Regex-based parser for Structurizr DSL syntax
- **Visualization Export**: Professional C4-compliant Graphviz DOT export
- **Anti-Pattern Detection**: Configurable detection for cycles, bottlenecks, over-coupling
- **Telemetry Infrastructure**: ML training data collection
- **Force-Directed Layout**: Fruchterman-Reingold algorithm implementation

### Improved
- **Documentation**: Phase 2.5 summary and integration roadmap
- **Build System**: Enhanced build instructions for JNI components
- **Performance**: Optimized layout calculations

---

## [1.05.0] - 2026-01-09 (Pre-release)

### Added
- **Licensing Clarity**: Consolidated LICENSE.md with AGPL-3.0/MIT/Apache-2.0
- **Copyright Attribution**: Tumquan Corp with Optaquan brand
- **Phase 2.5 Installation**: Parallel installation script
- **Branding**: OptaFly Phase 2.5 icon

### Security
- **Removed Temp-API-Key**: Eliminated sensitive data from repository
- **License Compliance**: Clarified AGPL-3.0 copyleft requirements

---

## [0.98.0] - 2026-01-08

### Added
- **OptaCore-Struct Foundation**: Tensor-based architecture modeling engine
- **Burn 0.19.1 Integration**: Machine learning framework
- **Phase 2 Extended Plan**: Architecture assistant roadmap

---

## [0.97.0] - 2026-01-07

### Added
- **Telemetry Collection System**: Usage tracking for ML training
- **PyO3 Bridge**: Python-Rust integration for Widget-Log
- **Widget-Log Integration**: Native semantic caching integration

---

## [0.96.0] - 2026-01-06

### Added
- **Multi-Agent Prompt Management**: Phase 2a implementation
- **Prompt Templates**: Structured prompt system
- **Agent Coordination**: Multi-agent collaboration framework

---

## [0.95.0] - 2026-01-05

### Added
- **Native Widget-Log Integration**: Semantic caching
- **280x Performance Boost**: 43ms vs 12,201ms response times
- **Semantic Similarity**: 95% accuracy in query matching
- **Localhost HTTPS Proxy**: Secure proxy on port 8443

### Security
- **Token Authentication**: 256-bit Bearer tokens
- **SSL/TLS Encryption**: Self-signed localhost certificates

---

## [0.90.0] - 2026-01-01

### Added
- **Initial OptaFly_Zed Fork**: Forked from Zed v0.217.4
- **Project Setup**: Basic structure and build configuration
- **Widget-Log Planning**: Architecture design

---

## Version Naming Convention

OptaFly_Zed uses semantic versioning (SemVer 2.0.0):
- **Major (1.x.x)**: Breaking changes, major feature releases
- **Minor (x.Y.x)**: New features, backward-compatible additions
- **Patch (x.x.Z)**: Bug fixes, minor improvements

### Version History:
- **v1.0.0**: Stable release with pinned dependencies and cross-platform installers
- **v0.99**: Phase 2.5 - ML Foundation + Structurizr JNI
- **v0.98**: Phase 2 Extended - OptaCore architecture assistant
- **v0.97**: Phase 2b - Telemetry and PyO3 bridge
- **v0.96**: Phase 2a - Multi-agent prompt management
- **v0.95**: Phase 2 - Native Widget-Log integration
- **v0.90**: Phase 1 - Initial fork and setup

---

## Future Roadmap

### Planned for v1.1.0 (Q1 2026)
- **Cargo Install**: Publish to crates.io as `optafly-zed`
- **Binary Releases**: Pre-built binaries for all platforms
- **Automatic Updates**: Built-in update mechanism
- **Usage Analytics**: Opt-in telemetry for improvement insights

### Planned for v1.2.0 (Q2 2026)
- **GPU Acceleration**: WGPU backend for large architecture layouts
- **Neural Layout Models**: ML-trained layout optimization
- **Advanced Anti-Patterns**: God objects, circular dependencies, tight coupling
- **Real-Time Collaboration**: Multi-user architecture editing

### Planned for v2.0.0 (Q3 2026)
- **Full Structurizr Compatibility**: Complete DSL and workspace support
- **Interactive Diagrams**: Live editing and real-time updates
- **Architecture Validation**: Rule-based constraint checking
- **Enterprise Features**: SSO, audit logs, compliance reporting

---

## Upstream Zed Synchronization

OptaFly_Zed periodically syncs with upstream Zed releases:

- **Current Base**: Zed v0.217.4 (2026-01-01)
- **Next Planned Sync**: Zed v0.220.x (Q1 2026)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

For licensing information, see [LICENSE.md](LICENSE.md).

---

## Links

- **Repository**: [Optaquan/OptaFly_Zed](https://github.com/Optaquan/OptaFly_Zed)
- **Issues**: [GitHub Issues](https://github.com/Optaquan/OptaFly_Zed/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Optaquan/OptaFly_Zed/discussions)
- **Upstream Zed**: [zed-industries/zed](https://github.com/zed-industries/zed)
- **Widget-Log**: [Optaquan/Widget-Log](https://github.com/Optaquan/Widget-Log)

---

**Copyright (c) 2025-2026 Tumquan Corp**  
**OptaFly_Zed is a derivative work of Zed editor**  
**Zed Copyright (c) 2022-2025 Zed Industries, Inc.**
