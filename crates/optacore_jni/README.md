# OptaCore JNI Bridge

JNI bridge enabling Java applications (especially Structurizr) to use OptaCore's Rust-based architecture optimization.

## Features

- **C4 DSL Parsing**: Convert Structurizr DSL to internal graph models
- **Force-Directed Layout**: GPU/CPU-accelerated Fruchterman-Reingold optimization
- **Anti-Pattern Detection**: Cycles, bottlenecks, over-coupling, isolated components
- **Graphviz Export**: Production-grade C4 diagram visualization
- **Panic Safety**: Rust panics converted to Java exceptions (no JVM crashes)
- **Cross-Platform**: Linux, macOS, Windows support with automatic native library loading

## Architecture

```
┌─────────────────┐
│ Structurizr     │  (Java/Kotlin)
│ Plugin/App      │
└────────┬────────┘
         │ JNI calls
         ▼
┌─────────────────┐
│ OptaCoreJNI.java│  (Java wrapper with platform detection)
└────────┬────────┘
         │ System.loadLibrary()
         ▼
┌─────────────────┐
│ liboptacore_jni │  (Rust cdylib)
│   .so/.dylib/.dll│
└────────┬────────┘
         │ Uses
         ▼
┌─────────────────┐
│ optacore_struct │  (Burn tensors + graph algorithms)
└─────────────────┘
```

## Building

### Prerequisites

- **Rust**: 1.75+ (with `cargo`)
- **Java JDK**: 11+ (for `javac`, testing)
- **System**: Linux, macOS, or Windows with build tools

### Build Commands

```bash
# Standard build (debug)
cargo build --package optacore_jni

# Release build (optimized, smaller binary)
cargo build --release --package optacore_jni

# Cross-compile for specific targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin  # Apple Silicon
cargo build --release --target x86_64-pc-windows-gnu

# Output location
# target/release/liboptacore_jni.so      (Linux)
# target/release/liboptacore_jni.dylib   (macOS)
# target/release/optacore_jni.dll        (Windows)
```

### Build with Custom JAVA_HOME

If the build fails to find JNI headers:

```bash
export JAVA_HOME=/usr/lib/jvm/java-11-openjdk-amd64
cargo build --release --package optacore_jni
```

### Testing

```bash
# Run Rust tests (validates config, debug logging, backend availability)
cargo test --release --package optacore_jni

# Expected output:
# running 3 tests
# test tests::test_backend_available ... ok
# test tests::test_config_validation ... ok
# test tests::test_debug_enabled ... ok
```

## Java Integration

### Step 1: Compile Java Wrapper

```bash
cd crates/optacore_jni/java
javac com/optafly/structurizr/OptaCoreJNI.java
```

### Step 2: Run Demo

```bash
# Set library path to Rust build output
export LD_LIBRARY_PATH=../../../target/release:$LD_LIBRARY_PATH

# Run Java demo
java -Djava.library.path=../../../target/release com.optafly.structurizr.OptaCoreJNI

# With debug logging
OPTACORE_JNI_DEBUG=1 java -Djava.library.path=../../../target/release com.optafly.structurizr.OptaCoreJNI
```

### Step 3: Bundle in JAR (Production)

#### Directory Structure

```
src/main/resources/native/
├── linux-x86_64/
│   └── liboptacore_jni.so
├── macos-x86_64/
│   └── liboptacore_jni.dylib
├── macos-aarch64/
│   └── liboptacore_jni.dylib
└── windows-x86_64/
    └── optacore_jni.dll
```

#### Maven Configuration

```xml
<build>
    <resources>
        <resource>
            <directory>src/main/resources</directory>
            <includes>
                <include>native/**/*</include>
            </includes>
        </resource>
    </resources>
</build>
```

#### Gradle Configuration

```gradle
sourceSets {
    main {
        resources {
            srcDirs = ['src/main/resources']
            include 'native/**/*'
        }
    }
}
```

The `OptaCoreJNI.java` class automatically extracts natives from JAR resources at runtime.

## API Reference

### JNI Methods (Rust → Java)

All methods follow the pattern:
- **Input**: JSON strings (UTF-8 encoded)
- **Output**: JSON string on success, `null` + exception on error
- **Thread Safety**: All methods are stateless and thread-safe

#### 1. `parseDsl(String dslInput) → String`

Parse C4 DSL into JSON model.

**Input DSL Example**:
```
system MySystem {
  container WebApp {
    component Frontend {}
    component Backend {}
  }
}
Frontend -> Backend "API calls"
```

**Output JSON**:
```json
{
  "nodes": [
    {
      "id": "Frontend",
      "name": "Frontend",
      "type": "Component",
      "position": [0.0, 0.0],
      "technology": null,
      "description": null
    }
  ],
  "edges": [
    {
      "from": "Frontend",
      "to": "Backend",
      "label": "API calls",
      "weight": 1.0
    }
  ],
  "node_count": 2,
  "edge_count": 1
}
```

**Errors**:
- Empty DSL input
- Invalid syntax
- UTF-8 encoding errors

---

#### 2. `optimizeLayout(String modelJson) → String`

Optimize node positions using force-directed layout (150 iterations).

**Input**: JSON from `parseDsl` (or manually constructed)

**Output**: Same structure with updated `position` fields + `optimization_stats`:
```json
{
  "nodes": [
    { "id": "A", "position": [12.34, 56.78], ... }
  ],
  "optimization_stats": {
    "iterations": 150,
    "duration_ms": 42
  }
}
```

**Configuration**:
- Algorithm: Fruchterman-Reingold
- Iterations: 150
- Initial temperature: 0.2
- Area: 2000.0
- Cooling schedule: Linear

**Errors**:
- Invalid JSON format
- Empty model
- Missing `nodes` or `edges` arrays

---

#### 3. `detectAntiPatterns(String modelJson, String configJson) → String`

Detect architectural anti-patterns.

**Config JSON** (optional, empty string uses defaults):
```json
{
  "bottleneck_threshold": 5,
  "over_coupling_threshold": 8
}
```

**Output**:
```json
{
  "patterns": [
    {
      "type": "Cycle",
      "nodes": ["A", "B", "C"],
      "severity": 1.0,
      "description": "Circular dependency detected: 3 nodes"
    },
    {
      "type": "Bottleneck",
      "node_id": "Database",
      "in_degree": 12,
      "severity": 0.85,
      "description": "High fan-in: 12 incoming edges"
    }
  ],
  "count": 2
}
```

**Pattern Types**:
- **Cycle**: severity=1.0 (critical)
- **Bottleneck**: severity=(in_degree - threshold) / threshold
- **OverCoupling**: severity=(out_degree - threshold) / threshold
- **IsolatedComponent**: severity=0.3

**Validation**:
- Thresholds must be > 0
- Throws `RuntimeException` if invalid

---

#### 4. `generateDot(String modelJson, String configJson) → String`

Generate Graphviz DOT visualization with C4 styling.

**Output** (DOT format):
```dot
digraph Architecture {
  bgcolor="#f8f8f8";
  splines=curved;
  node [style=filled, fontsize=10, fontcolor="#333333"];
  
  "Frontend" [label="Frontend", fillcolor="#aaddaa", shape=box, ...];
  "Backend" [label="Backend", fillcolor="#ddaaaa", shape=box, ...];
  
  "Frontend" -> "Backend" [label="API calls", penwidth=1.5];
}
```

**Features**:
- Node shapes by type (box3d=System, component=Container, box=Component, ellipse=Person)
- Severity-based colors (green #aaddaa → red #ff4444)
- Cycle highlighting (red edges, double borders)
- Isolated node dashed borders
- Bold fonts for high-severity (≥0.7)
- Soft background, curved edges, C4-compliant typography

**Render**:
```bash
dot -Tpng output.dot -o output.png
dot -Tsvg output.dot -o output.svg
```

---

#### 5. `getVersion() → String`

Returns crate version (e.g., `"0.1.0"`). Never fails.

---

### Java API (OptaCoreJNI.java)

#### Safe Call Wrapper

```java
import com.optafly.structurizr.OptaCoreJNI;

// Automatic null checking + exception on failure
String model = OptaCoreJNI.safeCall(() -> OptaCoreJNI.parseDsl(dsl));
```

#### Platform Detection

The Java class automatically:
1. Detects OS (Linux/macOS/Windows) and architecture (x86_64/aarch64)
2. Extracts native library from JAR resources to temp file
3. Falls back to system library path if JAR bundling not available
4. Honors explicit path via `-Doptacore.jni.library.path=/path/to/lib`

#### Error Handling

All JNI methods return `null` and throw `RuntimeException` on error:

```java
try {
    String result = OptaCoreJNI.parseDsl(invalidDsl);
} catch (RuntimeException e) {
    System.err.println("Parse error: " + e.getMessage());
    // e.g., "OptaCore error: DSL input is empty"
}
```

#### Memory Management

- **Local References**: Java GC handles cleanup automatically for typical usage
- **Long-Term Retention**: If holding JNI strings >1 second, copy to Java String immediately:
  ```java
  String result = new String(OptaCoreJNI.parseDsl(dsl)); // Force copy
  ```

---

## Structurizr Plugin Usage

### Kotlin Script Example

```kotlin
@file:DependsOn("com.optafly:optacore-jni:0.1.0")

import com.optafly.structurizr.OptaFlyPlugin
import com.structurizr.Workspace
import com.structurizr.dsl.StructurizrDslParser

val parser = StructurizrDslParser()
parser.parse(File("workspace.dsl"))
val workspace = parser.workspace

val plugin = OptaFlyPlugin(debugMode = true)

// Optimize layout for a specific view
plugin.optimizeLayout(workspace, "SystemContext")

// Detect anti-patterns workspace-wide
val patterns = plugin.detectAntiPatterns(workspace)
patterns.forEach { println(it) }

// Export DOT visualization
plugin.exportDot(workspace, "Container-WebApp", "output.dot")
```

### Gradle Plugin Integration

```kotlin
// build.gradle.kts
dependencies {
    implementation("com.structurizr:structurizr-core:1.28.1")
    implementation("com.optafly:optacore-jni:0.1.0")
}

tasks.register("optimizeArchitecture") {
    doLast {
        val plugin = com.optafly.structurizr.OptaFlyPlugin()
        // ... plugin.optimizeLayout(workspace, viewKey)
    }
}
```

---

## Performance

### Benchmarks (150 iterations, Intel i7-9750H)

| Nodes | Edges | Layout Time | Detection Time | DOT Export |
|-------|-------|-------------|----------------|------------|
| 10    | 15    | 8ms         | 2ms            | 1ms        |
| 50    | 100   | 42ms        | 8ms            | 5ms        |
| 100   | 200   | 95ms        | 15ms           | 12ms       |
| 500   | 1000  | 580ms       | 85ms           | 70ms       |

**Notes**:
- Layout scales O(n² × iterations) due to force calculations
- Detection is O(n + e) for cycles, O(n) for degree-based patterns
- GPU acceleration (with `wgpu` feature) provides 3-5x speedup for n > 500

---

## Debugging

### Enable Debug Logging

Set environment variable **before** loading the library:

```bash
export OPTACORE_JNI_DEBUG=1
java -Djava.library.path=./target/release YourApp
```

**Output** (to stderr):
```
[OptaCore JNI] Entering parseDsl
[OptaCore JNI] Parsing DSL (234 chars)
[OptaCore JNI] Parsed: 12 nodes, 18 edges
[OptaCore JNI] Success: 1456 bytes
[OptaCore JNI] Exiting parseDsl (normal)
```

### Common Issues

#### 1. `UnsatisfiedLinkError: no optacore_jni in java.library.path`

**Solution**:
```bash
# Add to library path
export LD_LIBRARY_PATH=/path/to/target/release:$LD_LIBRARY_PATH  # Linux
export DYLD_LIBRARY_PATH=/path/to/target/release:$DYLD_LIBRARY_PATH  # macOS

# Or use explicit path
java -Doptacore.jni.library.path=/path/to/liboptacore_jni.so YourApp
```

#### 2. `RuntimeException: Rust panic: ...`

**Cause**: Unhandled Rust panic (e.g., array index out of bounds, unwrap() failure)

**Solution**:
- Enable debug logging to see panic details
- Report as bug with stack trace

#### 3. `RuntimeException: JSON parse error: ...`

**Cause**: Invalid JSON format, missing required fields (`nodes`, `edges`)

**Solution**:
- Validate JSON with schema validator
- Check that node IDs in edges match actual node IDs

#### 4. `RuntimeException: Invalid bottleneck_threshold: 0 (must be > 0)`

**Cause**: Config validation failed

**Solution**:
- Ensure thresholds > 0
- Use empty string `""` for defaults

---

## Deployment Checklist

### For JAR Distribution

- [ ] Build natives for all target platforms (Linux x86_64, macOS x86_64/aarch64, Windows x86_64)
- [ ] Place in `src/main/resources/native/{os}-{arch}/` structure
- [ ] Verify JAR includes native libs: `jar tf yourapp.jar | grep native`
- [ ] Test on clean machine without system-installed libs
- [ ] Document minimum Java version (11+) in README
- [ ] Add shutdown hook for temp file cleanup (optional)

### For System Library

- [ ] Install `liboptacore_jni.so` to `/usr/local/lib` or `/usr/lib`
- [ ] Run `sudo ldconfig` (Linux) or add to `DYLD_LIBRARY_PATH` (macOS)
- [ ] Set `-Djava.library.path=/usr/local/lib` in application launcher
- [ ] Document installation steps for users

### Security Considerations

- **Code Signing**: Sign native libraries for macOS/Windows distribution
- **Dependency Audit**: Run `cargo audit` on Rust dependencies
- **Supply Chain**: Pin Burn version, verify checksums
- **Sandboxing**: JNI runs with full JVM privileges — validate inputs thoroughly

---

## Troubleshooting Build Errors

### `jni-sys` build failure

**Error**: `Could not find JNI headers`

**Fix**:
```bash
export JAVA_HOME=/path/to/jdk
cargo clean && cargo build --release --package optacore_jni
```

### Linking errors on macOS

**Error**: `ld: library not found for -lc++`

**Fix**:
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Use stable toolchain
rustup default stable
cargo build --release --package optacore_jni
```

### Cross-compilation issues

**Error**: `error: linker 'aarch64-linux-gnu-gcc' not found`

**Fix**:
```bash
# Install cross-compilation toolchain
sudo apt install gcc-aarch64-linux-gnu

# Configure Cargo
cat >> ~/.cargo/config.toml <<EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

cargo build --release --target aarch64-unknown-linux-gnu
```

---

## License

This JNI bridge follows the same license as OptaCore (check root LICENSE file).

**Dependencies**:
- `jni = "0.21"` (Apache-2.0/MIT)
- `optacore_struct` (internal)
- `burn = "0.19.1"` (Apache-2.0/MIT)

---

## Contributing

### Adding New JNI Methods

1. Add native method declaration in `OptaCoreJNI.java`:
   ```java
   public static native String myNewMethod(String input);
   ```

2. Implement in `src/lib.rs`:
   ```rust
   #[no_mangle]
   pub extern "system" fn Java_com_optafly_structurizr_OptaCoreJNI_myNewMethod(
       mut env: JNIEnv,
       _class: JClass,
       input: JString,
   ) -> jstring {
       with_panic_handler(&mut env, "myNewMethod", |env| {
           // Implementation
           Ok("result".to_string())
       })
   }
   ```

3. Test with `cargo test --package optacore_jni`

4. Update this README with API documentation

### Running Integration Tests

```bash
# Compile Java wrapper
cd crates/optacore_jni/java
javac com/optafly/structurizr/*.java

# Run Rust tests
cargo test --release --package optacore_jni

# Run Java demo (validates end-to-end)
java -Djava.library.path=../../../target/release com.optafly.structurizr.OptaCoreJNI
```

---

## Roadmap

- [ ] Maven Central deployment
- [ ] GraalVM native-image support
- [ ] Async JNI methods (CompletableFuture)
- [ ] JMH benchmarks
- [ ] Windows ARM64 support
- [ ] JNI Direct Buffer support for large graphs (zero-copy)

---

## Contact & Support

- **Issues**: [GitHub Issues](https://github.com/your-org/optafly/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/optafly/discussions)
- **Documentation**: [docs.rs/optacore_struct](https://docs.rs/optacore_struct)
