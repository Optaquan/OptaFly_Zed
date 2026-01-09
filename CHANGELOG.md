# Changelog

All notable changes to OptaFly_Zed will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.05.0] - 2026-01-09

### Added
- **Licensing Clarity**: Consolidated LICENSE.md with proper AGPL-3.0/MIT/Apache-2.0 licensing documentation
- **Copyright Attribution**: Updated copyright to Tumquan Corp with Optaquan brand acknowledgment
- **Phase 2.5 Installation**: Added parallel installation script (`install-phase25-parallel.sh`)
- **Documentation**: Added PHASE25_PARALLEL_INSTALL.md with complete setup guide
- **Branding**: Added OptaFly Phase 2.5 icon (`resources/icons/optafly-phase2.5.png`)

### Security
- **Removed Temp-API-Key directory**: Eliminated sensitive temporary API key storage from repository
- **License Compliance**: Clarified AGPL-3.0 copyleft requirements for network deployment

### Changed
- **License Documentation**: Reorganized licensing information for public release readiness
- **README**: Enhanced with commercial use guidance and licensing scenarios

---

## [0.99.0] - 2026-01-09 (Phase 2.5 Release)

### Added
- **Complete Phase 2.5**: ML Foundation + Structurizr JNI Integration
- **OptaCore JNI Bridge**: Native Java integration for Structurizr workspace analysis
- **C4 DSL Parser**: Regex-based parser for Structurizr DSL syntax
- **Visualization Export**: Professional C4-compliant Graphviz DOT export with:
  - Bold fonts and double borders for containers
  - Soft background colors (#E8F4F8)
  - Proper C4 notation and styling
- **Anti-Pattern Detection**: Configurable detection for cycles, bottlenecks, over-coupling
- **Telemetry Infrastructure**: ML training data collection for future neural layout models
- **Force-Directed Layout**: Fruchterman-Reingold algorithm implementation

### Improved
- **Documentation**: Phase 2.5 summary and integration roadmap
- **Build System**: Enhanced build instructions for JNI components
- **Performance**: Optimized layout calculations for large architectures

---

## [0.98.0] - 2026-01-08

### Added
- **OptaCore-Struct Foundation**: Tensor-based architecture modeling engine
- **Burn 0.19.1 Integration**: Machine learning framework for layout optimization
- **Phase 2 Extended Plan**: Architecture assistant development roadmap

### Changed
- **Project Structure**: Reorganized crates for OptaCore components
- **Dependencies**: Updated to latest Burn framework version

---

## [0.97.0] - 2026-01-07

### Added
- **Telemetry Collection System**: Comprehensive usage tracking for ML training
- **PyO3 Bridge**: Python-Rust integration for Widget-Log
- **Widget-Log Integration**: Complete native integration with semantic caching

### Improved
- **Performance Metrics**: Added detailed telemetry for cache hit rates and response times
- **Error Handling**: Enhanced error reporting in PyO3 bridge

---

## [0.96.0] - 2026-01-06

### Added
- **Multi-Agent Prompt Management**: Phase 2a implementation
- **Prompt Templates**: Structured prompt system for AI interactions
- **Agent Coordination**: Framework for multi-agent collaboration

### Improved
- **AI Response Quality**: Better prompt engineering for complex queries
- **Context Management**: Enhanced context handling for multi-turn conversations

---

## [0.95.0] - 2026-01-05

### Added
- **Native Widget-Log Integration**: First native integration of semantic caching
- **280x Performance Boost**: Cache hits achieve 43ms vs 12,201ms response times
- **Semantic Similarity**: 95% accuracy in detecting rephrased queries
- **Localhost HTTPS Proxy**: Secure proxy on port 8443 with authentication

### Security
- **Token Authentication**: 256-bit secure Bearer tokens for proxy access
- **SSL/TLS Encryption**: Self-signed certificates for localhost security

---

## [0.90.0] - 2026-01-01

### Added
- **Initial OptaFly_Zed Fork**: Forked from Zed v0.217.4
- **Project Setup**: Basic project structure and build configuration
- **Widget-Log Planning**: Architecture design for semantic caching integration

### Changed
- **Branding**: Rebranded as OptaFly_Zed with Optaquan identity
- **Build System**: Customized for OptaFly-specific features

---

## Version Naming Convention

OptaFly_Zed uses semantic versioning:
- **Major version (1.x.x)**: Breaking changes, major feature releases
- **Minor version (x.Y.x)**: New features, backward-compatible additions
- **Patch version (x.x.Z)**: Bug fixes, minor improvements

### Phase Mapping:
- **v1.05**: Public release preparation, licensing clarity
- **v0.99**: Phase 2.5 - ML Foundation + Structurizr JNI
- **v0.98**: Phase 2 Extended - OptaCore architecture assistant
- **v0.97**: Phase 2b - Telemetry and PyO3 bridge
- **v0.96**: Phase 2a - Multi-agent prompt management
- **v0.95**: Phase 2 - Native Widget-Log integration
- **v0.90**: Phase 1 - Initial fork and setup

---

## Future Roadmap

### Planned for v1.10.0 (Q1 2026)
- **GPU Acceleration**: WGPU backend for large architecture layouts (1000+ nodes)
- **Neural Layout Models**: ML-trained layout optimization
- **Advanced Anti-Patterns**: Detection of god objects, circular dependencies, tight coupling
- **Real-Time Collaboration**: Multi-user architecture editing

### Planned for v1.20.0 (Q2 2026)
- **Cloud Sync**: Optional cloud-based cache synchronization
- **Team Analytics**: Aggregated usage metrics and insights
- **Custom DSL Extensions**: User-defined architecture notation support
- **Export Formats**: PlantUML, Mermaid, and SVG export

### Planned for v2.0.0 (Q3 2026)
- **Full Structurizr Compatibility**: Complete DSL and workspace format support
- **Interactive Diagrams**: Live editing and real-time updates
- **Architecture Validation**: Rule-based architectural constraint checking
- **Enterprise Features**: SSO, audit logs, compliance reporting

---

## Upstream Zed Synchronization

OptaFly_Zed periodically syncs with upstream Zed releases:

- **Last Sync**: Zed v0.217.4 (2026-01-01)
- **Next Planned Sync**: Zed v0.220.x (Q1 2026)

Changes from upstream Zed are integrated while preserving OptaFly enhancements.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

For licensing information, see [LICENSE.md](LICENSE.md).

---

## Links

- **Repository**: [Optaquan/OptaFly_Zed](https://github.com/Optaquan/OptaFly_Zed)
- **Issues**: [GitHub Issues](https://github.com/Optaquan/OptaFly_Zed/issues)
- **Upstream Zed**: [zed-industries/zed](https://github.com/zed-industries/zed)
- **Widget-Log**: [Optaquan/Widget-Log](https://github.com/Optaquan/Widget-Log)

---

**Copyright (c) 2025-2026 Tumquan Corp**  
**OptaFly_Zed is a derivative work of Zed editor**  
**Zed Copyright (c) 2022-2025 Zed Industries, Inc.**
