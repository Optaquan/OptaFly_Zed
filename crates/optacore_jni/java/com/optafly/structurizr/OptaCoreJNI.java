package com.optafly.structurizr;

import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.function.Supplier;

/**
 * JNI Bridge to OptaCore Rust library for C4 architecture optimization.
 *
 * <p>Provides native methods for:
 * <ul>
 *   <li>Parsing C4 DSL into structured models</li>
 *   <li>Optimizing layout using force-directed algorithms</li>
 *   <li>Detecting architectural anti-patterns (cycles, bottlenecks, etc.)</li>
 *   <li>Generating Graphviz DOT visualizations</li>
 * </ul>
 *
 * <h2>Usage</h2>
 * <pre>{@code
 * String modelJson = OptaCoreJNI.parseDsl("system MySystem { ... }");
 * String optimized = OptaCoreJNI.optimizeLayout(modelJson);
 * String patterns = OptaCoreJNI.detectAntiPatterns(modelJson, "{}");
 * String dot = OptaCoreJNI.generateDot(modelJson, "");
 * }</pre>
 *
 * <h2>Error Handling</h2>
 * All methods return {@code null} on error and throw {@code RuntimeException}.
 * Use {@link #safeCall(Supplier)} for automatic null checks.
 *
 * <h2>Memory Management</h2>
 * JNI strings are automatically managed by GC for typical use cases.
 * For long-term retention, consider copying strings immediately.
 *
 * <h2>Debug Mode</h2>
 * Set environment variable {@code OPTACORE_JNI_DEBUG=1} before loading
 * the library to enable detailed Rust-side logging to stderr.
 *
 * @author OptaFly Team
 * @version 0.1.0
 */
public class OptaCoreJNI {

    private static boolean libraryLoaded = false;
    private static String libraryVersion = null;

    static {
        loadNativeLibrary();
    }

    /**
     * Load the native library with platform detection and fallback strategies.
     *
     * <p>Attempts to load in this order:
     * <ol>
     *   <li>Bundled native library from JAR resources</li>
     *   <li>System library path (LD_LIBRARY_PATH, etc.)</li>
     *   <li>Explicit path from {@code optacore.jni.library.path} system property</li>
     * </ol>
     *
     * @throws UnsatisfiedLinkError if library cannot be loaded
     */
    private static synchronized void loadNativeLibrary() {
        if (libraryLoaded) {
            return;
        }

        String explicitPath = System.getProperty("optacore.jni.library.path");
        if (explicitPath != null && !explicitPath.isEmpty()) {
            try {
                System.load(explicitPath);
                libraryLoaded = true;
                libraryVersion = getVersion();
                System.err.println("[OptaCore JNI] Loaded from explicit path: " + explicitPath +
                                 " (version " + libraryVersion + ")");
                return;
            } catch (UnsatisfiedLinkError e) {
                System.err.println("[OptaCore JNI] Failed to load from explicit path: " + e.getMessage());
            }
        }

        try {
            String os = getOSName();
            String arch = getArchName();
            String libName = getLibraryName(os);
            String resourcePath = "/native/" + os + "-" + arch + "/" + libName;

            InputStream is = OptaCoreJNI.class.getResourceAsStream(resourcePath);
            if (is != null) {
                Path tempFile = Files.createTempFile("liboptacore_jni", getLibrarySuffix(os));
                tempFile.toFile().deleteOnExit();
                Files.copy(is, tempFile, StandardCopyOption.REPLACE_EXISTING);
                is.close();

                System.load(tempFile.toAbsolutePath().toString());
                libraryLoaded = true;
                libraryVersion = getVersion();
                System.err.println("[OptaCore JNI] Loaded from JAR: " + resourcePath +
                                 " (version " + libraryVersion + ")");
                return;
            }
        } catch (Exception e) {
            System.err.println("[OptaCore JNI] Failed to load from JAR: " + e.getMessage());
        }

        try {
            System.loadLibrary("optacore_jni");
            libraryLoaded = true;
            libraryVersion = getVersion();
            System.err.println("[OptaCore JNI] Loaded from system path (version " + libraryVersion + ")");
        } catch (UnsatisfiedLinkError e) {
            throw new UnsatisfiedLinkError(
                "Failed to load liboptacore_jni. Tried:\n" +
                "  1. Bundled JAR resources\n" +
                "  2. System library path\n" +
                "  3. Explicit path (-Doptacore.jni.library.path=...)\n" +
                "Original error: " + e.getMessage()
            );
        }
    }

    private static String getOSName() {
        String os = System.getProperty("os.name").toLowerCase();
        if (os.contains("win")) return "windows";
        if (os.contains("mac") || os.contains("darwin")) return "macos";
        if (os.contains("nix") || os.contains("nux")) return "linux";
        return "unknown";
    }

    private static String getArchName() {
        String arch = System.getProperty("os.arch").toLowerCase();
        if (arch.contains("amd64") || arch.contains("x86_64")) return "x86_64";
        if (arch.contains("aarch64") || arch.contains("arm64")) return "aarch64";
        return arch;
    }

    private static String getLibraryName(String os) {
        switch (os) {
            case "windows": return "optacore_jni.dll";
            case "macos": return "liboptacore_jni.dylib";
            default: return "liboptacore_jni.so";
        }
    }

    private static String getLibrarySuffix(String os) {
        switch (os) {
            case "windows": return ".dll";
            case "macos": return ".dylib";
            default: return ".so";
        }
    }

    /**
     * Parse C4 DSL text into a JSON model.
     *
     * @param dslInput C4 DSL string (e.g., "system Foo { container Bar { } }")
     * @return JSON object with "nodes", "edges", "node_count", "edge_count"
     * @throws RuntimeException if DSL is invalid or parsing fails
     */
    public static native String parseDsl(String dslInput);

    /**
     * Optimize layout for a JSON model using force-directed layout.
     *
     * <p>Runs 150 iterations of Fruchterman-Reingold with cooling.
     * Updates node positions in place.
     *
     * @param modelJson JSON model from {@link #parseDsl(String)}
     * @return Updated JSON with optimized "position" fields + "optimization_stats"
     * @throws RuntimeException if model JSON is invalid or optimization fails
     */
    public static native String optimizeLayout(String modelJson);

    /**
     * Detect architectural anti-patterns in a model.
     *
     * <p>Detects:
     * <ul>
     *   <li><b>Cycle</b>: Circular dependencies</li>
     *   <li><b>Bottleneck</b>: High fan-in (many incoming edges)</li>
     *   <li><b>OverCoupling</b>: High fan-out (many outgoing edges)</li>
     *   <li><b>IsolatedComponent</b>: No connections</li>
     * </ul>
     *
     * @param modelJson JSON model from {@link #parseDsl(String)}
     * @param configJson Config JSON with optional "bottleneck_threshold", "overcoupling_threshold"
     *                   (empty string "" uses defaults: 5, 8)
     * @return JSON object with "patterns" array and "count"
     * @throws RuntimeException if JSON is invalid, config thresholds are ≤ 0, or detection fails
     */
    public static native String detectAntiPatterns(String modelJson, String configJson);

    /**
     * Generate Graphviz DOT visualization.
     *
     * <p>Produces production-grade C4 diagrams with:
     * <ul>
     *   <li>Node shapes by type (box3d, component, box, ellipse)</li>
     *   <li>Severity-based color gradients (green → yellow → red)</li>
     *   <li>Cycle highlighting (red edges, double borders)</li>
     *   <li>Isolated node dashed borders</li>
     *   <li>Curved edges, soft background, typography</li>
     * </ul>
     *
     * @param modelJson JSON model from {@link #parseDsl(String)}
     * @param configJson Config JSON for anti-pattern thresholds (empty string "" uses defaults)
     * @return DOT string ready for `dot -Tpng output.dot -o output.png`
     * @throws RuntimeException if JSON is invalid or generation fails
     */
    public static native String generateDot(String modelJson, String configJson);

    /**
     * Get native library version.
     *
     * @return Version string (e.g., "0.1.0")
     */
    public static native String getVersion();

    /**
     * Health check - verifies native library is functional.
     *
     * <p>Tests backend initialization without throwing exceptions.
     * Useful for startup validation and diagnostics.
     *
     * @return true if OptaCore backend is available and functional
     */
    public static native boolean healthCheck();

    /**
     * Safe JNI call wrapper with automatic null checking.
     *
     * <p>Throws {@code RuntimeException} if the JNI call returns null
     * (indicating a Rust panic or error was logged to stderr).
     *
     * <h3>Example</h3>
     * <pre>{@code
     * String model = OptaCoreJNI.safeCall(() -> OptaCoreJNI.parseDsl(dsl));
     * }</pre>
     *
     * @param jniCall supplier wrapping a native method call
     * @return non-null result from JNI
     * @throws RuntimeException if JNI call returns null (check logs for Rust panic details)
     */
    public static String safeCall(Supplier<String> jniCall) {
        String result = jniCall.get();
        if (result == null) {
            throw new RuntimeException(
                "JNI call failed — returned null. Check stderr/logcat for Rust panic details. " +
                "Enable debug logging with OPTACORE_JNI_DEBUG=1."
            );
        }
        return result;
    }

    /**
     * Check if the native library is loaded and functional.
     *
     * @return true if library is loaded
     */
    public static boolean isLibraryLoaded() {
        return libraryLoaded;
    }

    /**
     * Get the loaded library version (cached from first {@link #getVersion()} call).
     *
     * @return version string or null if library not loaded
     */
    public static String getLibraryVersion() {
        return libraryVersion;
    }

    /**
     * Example: Full workflow from DSL to DOT visualization.
     *
     * @param args command-line arguments (unused)
     */
    public static void main(String[] args) {
        System.out.println("OptaCore JNI Version: " + getLibraryVersion());
        System.out.println("Library loaded: " + isLibraryLoaded());

        String dsl = "system MySystem {\n" +
                     "  container WebApp {\n" +
                     "    component Frontend {}\n" +
                     "    component Backend {}\n" +
                     "  }\n" +
                     "  container Database {}\n" +
                     "}\n" +
                     "Frontend -> Backend \"API calls\"\n" +
                     "Backend -> Database \"queries\"";

        try {
            System.out.println("\n1. Parsing DSL...");
            String modelJson = safeCall(() -> parseDsl(dsl));
            System.out.println("Model: " + modelJson.substring(0, Math.min(100, modelJson.length())) + "...");

            System.out.println("\n2. Optimizing layout...");
            String optimized = safeCall(() -> optimizeLayout(modelJson));
            System.out.println("Optimized: " + optimized.substring(0, Math.min(100, optimized.length())) + "...");

            System.out.println("\n3. Detecting anti-patterns...");
            String patterns = safeCall(() -> detectAntiPatterns(optimized, ""));
            System.out.println("Patterns: " + patterns);

            System.out.println("\n4. Generating DOT visualization...");
            String dot = safeCall(() -> generateDot(optimized, ""));
            System.out.println("DOT output (" + dot.length() + " bytes):\n" + dot.substring(0, Math.min(300, dot.length())) + "...");

        } catch (RuntimeException e) {
            System.err.println("Error: " + e.getMessage());
            e.printStackTrace();
        }
    }
}
