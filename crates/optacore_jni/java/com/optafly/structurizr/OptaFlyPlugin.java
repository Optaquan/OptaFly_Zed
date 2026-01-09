package com.optafly.structurizr;

import com.structurizr.Workspace;
import com.structurizr.model.*;
import com.structurizr.view.*;

import org.json.JSONArray;
import org.json.JSONObject;

import java.util.*;

/**
 * Structurizr plugin that integrates OptaCore force-directed layout optimization.
 *
 * <p>This plugin enhances Structurizr workspaces by:
 * <ul>
 *   <li>Converting C4 models to DSL for OptaCore processing</li>
 *   <li>Optimizing diagram layouts using Rust-based force-directed algorithms</li>
 *   <li>Detecting architectural anti-patterns and annotating diagrams</li>
 *   <li>Exporting production-grade Graphviz visualizations</li>
 * </ul>
 *
 * <h2>Usage in Structurizr DSL</h2>
 * <pre>{@code
 * workspace {
 *   model {
 *     system = softwareSystem "MySystem" {
 *       webapp = container "WebApp"
 *       db = container "Database"
 *     }
 *     webapp -> db "Reads from"
 *   }
 *   views {
 *     systemContext system {
 *       include *
 *       autoLayout  // Replace with OptaFly optimization
 *     }
 *   }
 * }
 * }</pre>
 *
 * <h2>Kotlin Script Integration</h2>
 * <pre>{@code
 * import com.optafly.structurizr.OptaFlyPlugin
 *
 * val workspace = Workspace("My Workspace", "Description")
 * // ... build model ...
 *
 * val plugin = OptaFlyPlugin()
 * plugin.optimizeLayout(workspace, "SystemContext")
 * plugin.detectAntiPatterns(workspace).forEach { println(it) }
 * plugin.exportDot(workspace, "SystemContext", "output.dot")
 * }</pre>
 *
 * @author OptaFly Team
 * @version 0.1.0
 */
public class OptaFlyPlugin {

    private static final String VERSION = "0.1.0";
    private boolean debugMode;

    /**
     * Create a new OptaFly plugin instance.
     */
    public OptaFlyPlugin() {
        this(false);
    }

    /**
     * Create a new OptaFly plugin instance with debug mode.
     *
     * @param debugMode if true, enable verbose logging
     */
    public OptaFlyPlugin(boolean debugMode) {
        this.debugMode = debugMode;
        if (!OptaCoreJNI.isLibraryLoaded()) {
            throw new IllegalStateException("OptaCore JNI library not loaded");
        }
        log("Initialized OptaFly Plugin v" + VERSION +
            " with OptaCore v" + OptaCoreJNI.getLibraryVersion());
    }

    /**
     * Optimize layout for a specific view in a Structurizr workspace.
     *
     * <p>This replaces Structurizr's default {@code autoLayout} with force-directed optimization.
     * Node positions are updated in place.
     *
     * @param workspace Structurizr workspace
     * @param viewKey view key (e.g., "SystemContext", "Container-WebApp")
     * @throws IllegalArgumentException if view not found
     * @throws RuntimeException if optimization fails
     */
    public void optimizeLayout(Workspace workspace, String viewKey) {
        log("Optimizing layout for view: " + viewKey);

        View view = findView(workspace, viewKey);
        if (view == null) {
            throw new IllegalArgumentException("View not found: " + viewKey);
        }

        String dsl = convertViewToDSL(view);
        log("Generated DSL (" + dsl.length() + " chars)");

        String modelJson = OptaCoreJNI.safeCall(() -> OptaCoreJNI.parseDsl(dsl));
        log("Parsed model: " + modelJson.substring(0, Math.min(100, modelJson.length())));

        String optimized = OptaCoreJNI.safeCall(() -> OptaCoreJNI.optimizeLayout(modelJson));
        log("Optimization complete");

        applyPositionsToView(view, optimized);
        log("Applied positions to view");
    }

    /**
     * Detect anti-patterns across the entire workspace model.
     *
     * @param workspace Structurizr workspace
     * @return list of detected anti-patterns with descriptions
     * @throws RuntimeException if detection fails
     */
    public List<AntiPattern> detectAntiPatterns(Workspace workspace) {
        log("Detecting anti-patterns in workspace: " + workspace.getName());

        String dsl = convertModelToDSL(workspace.getModel());
        log("Generated workspace DSL (" + dsl.length() + " chars)");

        String modelJson = OptaCoreJNI.safeCall(() -> OptaCoreJNI.parseDsl(dsl));

        String patternsJson = OptaCoreJNI.safeCall(() ->
            OptaCoreJNI.detectAntiPatterns(modelJson, "")
        );

        List<AntiPattern> patterns = parseAntiPatterns(patternsJson);
        log("Detected " + patterns.size() + " anti-patterns");

        return patterns;
    }

    /**
     * Export a view as Graphviz DOT format.
     *
     * @param workspace Structurizr workspace
     * @param viewKey view key
     * @param outputPath output file path (e.g., "output.dot")
     * @throws IllegalArgumentException if view not found
     * @throws RuntimeException if export fails
     */
    public void exportDot(Workspace workspace, String viewKey, String outputPath) {
        log("Exporting view to DOT: " + viewKey + " -> " + outputPath);

        View view = findView(workspace, viewKey);
        if (view == null) {
            throw new IllegalArgumentException("View not found: " + viewKey);
        }

        String dsl = convertViewToDSL(view);
        String modelJson = OptaCoreJNI.safeCall(() -> OptaCoreJNI.parseDsl(dsl));
        String optimized = OptaCoreJNI.safeCall(() -> OptaCoreJNI.optimizeLayout(modelJson));

        String dot = OptaCoreJNI.safeCall(() -> OptaCoreJNI.generateDot(optimized, ""));

        try {
            java.nio.file.Files.writeString(java.nio.file.Path.of(outputPath), dot);
            log("Exported DOT (" + dot.length() + " bytes) to " + outputPath);
        } catch (java.io.IOException e) {
            throw new RuntimeException("Failed to write DOT file: " + e.getMessage(), e);
        }
    }

    /**
     * Convert a Structurizr view to C4 DSL for OptaCore.
     *
     * <p>Maps Structurizr elements to OptaCore node types:
     * <ul>
     *   <li>{@code SoftwareSystem} → System</li>
     *   <li>{@code Container} → Container</li>
     *   <li>{@code Component} → Component</li>
     *   <li>{@code Person} → Person</li>
     * </ul>
     *
     * @param view Structurizr view
     * @return C4 DSL string
     */
    private String convertViewToDSL(View view) {
        StringBuilder dsl = new StringBuilder();
        Set<Element> elements = new HashSet<>();
        Set<Relationship> relationships = new HashSet<>();

        if (view instanceof ModelView) {
            ModelView modelView = (ModelView) view;
            modelView.getElements().forEach(ev -> elements.add(ev.getElement()));
            modelView.getRelationships().forEach(rv -> relationships.add(rv.getRelationship()));
        }

        Map<String, String> elementIds = new HashMap<>();
        int idCounter = 0;

        for (Element element : elements) {
            String nodeId = "node_" + (idCounter++);
            elementIds.put(element.getId(), nodeId);

            String nodeType = getNodeType(element);
            String name = sanitize(element.getName());
            String technology = element.getProperties().getOrDefault("technology", "");
            String description = sanitize(element.getDescription());

            dsl.append(nodeType).append(" ").append(nodeId)
               .append(" \"").append(name).append("\" {");
            if (!technology.isEmpty()) {
                dsl.append(" technology \"").append(technology).append("\"");
            }
            if (!description.isEmpty()) {
                dsl.append(" description \"").append(description).append("\"");
            }
            dsl.append(" }\n");
        }

        for (Relationship rel : relationships) {
            String fromId = elementIds.get(rel.getSource().getId());
            String toId = elementIds.get(rel.getDestination().getId());
            if (fromId != null && toId != null) {
                String label = sanitize(rel.getDescription());
                dsl.append(fromId).append(" -> ").append(toId);
                if (!label.isEmpty()) {
                    dsl.append(" \"").append(label).append("\"");
                }
                dsl.append("\n");
            }
        }

        return dsl.toString();
    }

    /**
     * Convert entire Structurizr model to DSL (for workspace-wide anti-pattern detection).
     */
    private String convertModelToDSL(Model model) {
        StringBuilder dsl = new StringBuilder();
        Map<String, String> elementIds = new HashMap<>();
        int idCounter = 0;

        for (SoftwareSystem system : model.getSoftwareSystems()) {
            String systemId = "sys_" + (idCounter++);
            elementIds.put(system.getId(), systemId);
            dsl.append("system ").append(systemId)
               .append(" \"").append(sanitize(system.getName())).append("\" {}\n");

            for (Container container : system.getContainers()) {
                String containerId = "container_" + (idCounter++);
                elementIds.put(container.getId(), containerId);
                dsl.append("container ").append(containerId)
                   .append(" \"").append(sanitize(container.getName())).append("\" {}\n");

                for (Component component : container.getComponents()) {
                    String componentId = "comp_" + (idCounter++);
                    elementIds.put(component.getId(), componentId);
                    dsl.append("component ").append(componentId)
                       .append(" \"").append(sanitize(component.getName())).append("\" {}\n");
                }
            }
        }

        for (Person person : model.getPeople()) {
            String personId = "person_" + (idCounter++);
            elementIds.put(person.getId(), personId);
            dsl.append("person ").append(personId)
               .append(" \"").append(sanitize(person.getName())).append("\" {}\n");
        }

        for (Relationship rel : model.getRelationships()) {
            String fromId = elementIds.get(rel.getSource().getId());
            String toId = elementIds.get(rel.getDestination().getId());
            if (fromId != null && toId != null) {
                dsl.append(fromId).append(" -> ").append(toId)
                   .append(" \"").append(sanitize(rel.getDescription())).append("\"\n");
            }
        }

        return dsl.toString();
    }

    private String getNodeType(Element element) {
        if (element instanceof SoftwareSystem) return "system";
        if (element instanceof Container) return "container";
        if (element instanceof Component) return "component";
        if (element instanceof Person) return "person";
        return "system";
    }

    private String sanitize(String text) {
        if (text == null) return "";
        return text.replace("\"", "'").replace("\n", " ").trim();
    }

    private View findView(Workspace workspace, String viewKey) {
        ViewSet views = workspace.getViews();
        for (SystemLandscapeView view : views.getSystemLandscapeViews()) {
            if (view.getKey().equals(viewKey)) return view;
        }
        for (SystemContextView view : views.getSystemContextViews()) {
            if (view.getKey().equals(viewKey)) return view;
        }
        for (ContainerView view : views.getContainerViews()) {
            if (view.getKey().equals(viewKey)) return view;
        }
        for (ComponentView view : views.getComponentViews()) {
            if (view.getKey().equals(viewKey)) return view;
        }
        return null;
    }

    private void applyPositionsToView(View view, String optimizedJson) {
        JSONObject json = new JSONObject(optimizedJson);
        JSONArray nodes = json.getJSONArray("nodes");

        if (!(view instanceof ModelView)) return;
        ModelView modelView = (ModelView) view;

        Map<String, JSONObject> nodeMap = new HashMap<>();
        for (int i = 0; i < nodes.length(); i++) {
            JSONObject node = nodes.getJSONObject(i);
            nodeMap.put(node.getString("id"), node);
        }

        for (ElementView ev : modelView.getElements()) {
            JSONObject node = nodeMap.get(ev.getElement().getId());
            if (node != null && node.has("position")) {
                JSONArray pos = node.getJSONArray("position");
                ev.setX((int) (pos.getDouble(0) * 100));
                ev.setY((int) (pos.getDouble(1) * 100));
            }
        }
    }

    private List<AntiPattern> parseAntiPatterns(String json) {
        List<AntiPattern> patterns = new ArrayList<>();
        JSONObject obj = new JSONObject(json);
        JSONArray patternsArray = obj.getJSONArray("patterns");

        for (int i = 0; i < patternsArray.length(); i++) {
            JSONObject p = patternsArray.getJSONObject(i);
            String type = p.getString("type");
            double severity = p.getDouble("severity");
            String description = p.optString("description", "");

            AntiPattern pattern = new AntiPattern(type, severity, description);
            if (p.has("node_id")) {
                pattern.nodeId = p.getString("node_id");
            }
            if (p.has("nodes")) {
                JSONArray nodesArray = p.getJSONArray("nodes");
                for (int j = 0; j < nodesArray.length(); j++) {
                    pattern.nodes.add(nodesArray.getString(j));
                }
            }
            patterns.add(pattern);
        }

        return patterns;
    }

    private void log(String message) {
        if (debugMode) {
            System.err.println("[OptaFly Plugin] " + message);
        }
    }

    /**
     * Anti-pattern detected by OptaCore.
     */
    public static class AntiPattern {
        public final String type;
        public final double severity;
        public final String description;
        public String nodeId;
        public List<String> nodes = new ArrayList<>();

        public AntiPattern(String type, double severity, String description) {
            this.type = type;
            this.severity = severity;
            this.description = description;
        }

        @Override
        public String toString() {
            return String.format("[%s] %s (severity: %.2f) - %s",
                type, nodeId != null ? nodeId : nodes, severity, description);
        }
    }

    /**
     * Example: Standalone plugin usage.
     */
    public static void main(String[] args) {
        OptaFlyPlugin plugin = new OptaFlyPlugin(true);

        Workspace workspace = new Workspace("Example", "Demo workspace");
        Model model = workspace.getModel();

        SoftwareSystem system = model.addSoftwareSystem("MySystem", "Example system");
        Container webapp = system.addContainer("WebApp", "Web application", "Java");
        Container db = system.addContainer("Database", "Data store", "PostgreSQL");
        webapp.uses(db, "Reads from and writes to", "JDBC");

        SystemContextView view = workspace.getViews().createSystemContextView(
            system, "SystemContext", "System context diagram"
        );
        view.addAllElements();

        plugin.optimizeLayout(workspace, "SystemContext");
        List<AntiPattern> patterns = plugin.detectAntiPatterns(workspace);
        patterns.forEach(System.out::println);

        plugin.exportDot(workspace, "SystemContext", "example_output.dot");
        System.out.println("Done! Check example_output.dot");
    }
}
