# OptaCore JNI - Quick Start Guide

**5-Minute Integration Guide for Java/Kotlin Developers**

---

## 🚀 Installation

### Option 1: Build from Source

```bash
# Clone repo and build JNI library
git clone https://github.com/your-org/optafly.git
cd optafly
cargo build --release --package optacore_jni

# Library location: target/release/liboptacore_jni.so (Linux)
```

### Option 2: Download Pre-Built (Future)

```bash
# Coming soon: Maven Central
<dependency>
    <groupId>com.optafly</groupId>
    <artifactId>optacore-jni</artifactId>
    <version>0.1.0</version>
</dependency>
```

---

## 📝 Hello World (Java)

```java
import com.optafly.structurizr.OptaCoreJNI;

public class HelloOptaCore {
    public static void main(String[] args) {
        // System.loadLibrary("optacore_jni") called automatically
        
        String dsl = """
            system MySystem {
              container WebApp {}
              container Database {}
            }
            WebApp -> Database "queries"
            """;
        
        // 1. Parse DSL
        String modelJson = OptaCoreJNI.parseDsl(dsl);
        System.out.println("Model: " + modelJson);
        
        // 2. Optimize layout
        String optimized = OptaCoreJNI.optimizeLayout(modelJson);
        System.out.println("Optimized!");
        
        // 3. Detect anti-patterns
        String patterns = OptaCoreJNI.detectAntiPatterns(optimized, "");
        System.out.println("Patterns: " + patterns);
        
        // 4. Export visualization
        String dot = OptaCoreJNI.generateDot(optimized, "");
        System.out.println(dot);
    }
}
```

**Run**:
```bash
javac HelloOptaCore.java
java -Djava.library.path=./target/release HelloOptaCore
```

---

## 🔧 Structurizr Plugin (Kotlin)

```kotlin
import com.optafly.structurizr.OptaFlyPlugin
import com.structurizr.Workspace
import com.structurizr.api.StructurizrClient

fun main() {
    // Load workspace
    val workspace = Workspace("My Workspace", "Description")
    val model = workspace.model
    
    val system = model.addSoftwareSystem("MySystem")
    val webapp = system.addContainer("WebApp", "Web UI", "React")
    val api = system.addContainer("API", "Backend", "Kotlin")
    webapp.uses(api, "Calls")
    
    workspace.views.createSystemContextView(system, "context", "System Context")
        .addAllElements()
    
    // Optimize with OptaCore
    val plugin = OptaFlyPlugin(debugMode = true)
    plugin.optimizeLayout(workspace, "context")
    
    // Detect issues
    plugin.detectAntiPatterns(workspace).forEach {
        println("⚠️  [${it.type}] ${it.description}")
    }
    
    // Export
    plugin.exportDot(workspace, "context", "diagram.dot")
    Runtime.getRuntime().exec("dot -Tpng diagram.dot -o diagram.png")
}
```

---

## 🎯 Common Patterns

### Pattern 1: Safe Error Handling

```java
import com.optafly.structurizr.OptaCoreJNI;

try {
    String result = OptaCoreJNI.safeCall(() -> 
        OptaCoreJNI.parseDsl(userInputDsl)
    );
} catch (RuntimeException e) {
    logger.error("Parse failed: {}", e.getMessage());
    // Handle gracefully
}
```

### Pattern 2: Custom Config

```java
String config = """
    {
      "bottleneck_threshold": 3,
      "over_coupling_threshold": 6
    }
    """;

String patterns = OptaCoreJNI.detectAntiPatterns(modelJson, config);
```

### Pattern 3: Batch Processing

```java
List<String> dsls = loadDslFiles();

for (String dsl : dsls) {
    String model = OptaCoreJNI.parseDsl(dsl);
    String optimized = OptaCoreJNI.optimizeLayout(model);
    String dot = OptaCoreJNI.generateDot(optimized, "");
    saveDot(dot, "output_" + i + ".dot");
}
```

---

## 🐛 Debugging

### Enable Verbose Logging

```bash
export OPTACORE_JNI_DEBUG=1
java -Djava.library.path=./target/release YourApp

# Output:
# [OptaCore JNI] Entering parseDsl
# [OptaCore JNI] Parsing DSL (123 chars)
# [OptaCore JNI] Parsed: 5 nodes, 7 edges
# [OptaCore JNI] Success: 456 bytes
```

### Check Version

```java
System.out.println("OptaCore version: " + OptaCoreJNI.getVersion());
// Output: 0.1.0
```

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `UnsatisfiedLinkError` | Library not found | Set `-Djava.library.path=...` |
| `Rust panic: ...` | Internal error | Enable debug logging, report bug |
| `JSON parse error` | Invalid JSON | Validate with schema |
| `threshold: 0 (must be > 0)` | Bad config | Use positive thresholds |

---

## 📦 Deployment

### JAR with Bundled Natives

```
myapp.jar
├── META-INF/
├── com/mycompany/...
└── native/
    ├── linux-x86_64/liboptacore_jni.so
    ├── macos-x86_64/liboptacore_jni.dylib
    └── windows-x86_64/optacore_jni.dll
```

`OptaCoreJNI.java` auto-extracts and loads the correct native lib.

---

## ⚡ Performance Tips

1. **Reuse Models**: Parse once, optimize multiple times
   ```java
   String model = OptaCoreJNI.parseDsl(dsl);
   for (int i = 0; i < runs; i++) {
       String optimized = OptaCoreJNI.optimizeLayout(model);
   }
   ```

2. **Parallel Processing**: JNI calls are thread-safe
   ```java
   List<String> models = ...;
   models.parallelStream()
       .map(OptaCoreJNI::optimizeLayout)
       .collect(Collectors.toList());
   ```

3. **Large Graphs** (>500 nodes): Build with GPU support
   ```bash
   cargo build --release --features wgpu --package optacore_jni
   ```

---

## 📚 API Quick Reference

| Method | Purpose | Input | Output |
|--------|---------|-------|--------|
| `parseDsl(dsl)` | Parse DSL | C4 DSL string | JSON model |
| `optimizeLayout(json)` | Optimize | JSON model | JSON with positions |
| `detectAntiPatterns(json, config)` | Find issues | JSON + config | JSON patterns |
| `generateDot(json, config)` | Visualize | JSON + config | DOT string |
| `getVersion()` | Version | - | "0.1.0" |

**All methods**:
- Return `null` + throw `RuntimeException` on error
- Are stateless and thread-safe
- Auto-handle memory management

---

## 🔗 Links

- [Full Documentation](./README.md)
- [Implementation Summary](../../JNI_BRIDGE_SUMMARY.md)
- [Structurizr Docs](https://docs.structurizr.com)
- [Burn Framework](https://burn.dev)
- [C4 Model](https://c4model.com)

---

## 💡 Tips

- Use `OptaCoreJNI.safeCall(...)` for automatic null checking
- Set `OPTACORE_JNI_DEBUG=1` when troubleshooting
- Call `getVersion()` on startup to verify library loaded
- Empty config string `""` uses sensible defaults
- Render DOT with: `dot -Tpng input.dot -o output.png`

---

**Ready to integrate? Check [README.md](./README.md) for detailed docs!**
