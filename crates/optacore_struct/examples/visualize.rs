use burn::backend::ndarray::NdArray;
use optacore_struct::{
    AntiPatternConfig, OptaOptimizer, parse_c4_dsl, to_dot, to_dot_with_positions,
};
use std::fs;

type Backend = NdArray<f32>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OptaCore Visualization Demo");
    println!("============================\n");

    let dsl = r#"
container Frontend "Frontend App" "React"
container API "API Gateway" "Node.js"
container AuthService "Auth Service" "Java"
container UserDB "User Database" "PostgreSQL"
container CacheService "Cache" "Redis"
container NotificationService "Notifications" "Python"
container MetricsCollector "Metrics" "Go"
container LogAggregator "Logs" "Elasticsearch"

Frontend -> API "requests"
API -> AuthService "authenticates"
API -> CacheService "caches"
API -> NotificationService "sends"
API -> MetricsCollector "tracks"
API -> LogAggregator "logs"
AuthService -> UserDB "queries"
AuthService -> CacheService "sessions"
NotificationService -> API "polls"
CacheService -> UserDB "reads"
MetricsCollector -> API "monitors"
LogAggregator -> API "indexes"
"#;

    println!("Parsing C4 DSL...");
    let mut model = parse_c4_dsl::<Backend>(dsl)?;
    println!(
        "✓ Parsed {} nodes, {} edges\n",
        model.node_count(),
        model.edge_count()
    );

    println!("Building adjacency matrix...");
    model.build_adjacency_matrix();
    println!("✓ Adjacency matrix built\n");

    println!("Running layout optimization...");
    let optimizer = OptaOptimizer::<Backend>::new(150, 0.2).with_area(2000.0);
    optimizer.optimize_layout(&mut model)?;
    println!("✓ Layout optimized\n");

    println!("Detecting anti-patterns for visualization...");
    let config = AntiPatternConfig {
        bottleneck_threshold: 4,
        over_coupling_threshold: 5,
        detect_isolated: true,
    };

    println!("Generating DOT visualizations...\n");

    let dot_with_anti_patterns = to_dot(&model, Some(&config))?;
    fs::write("architecture_with_patterns.dot", &dot_with_anti_patterns)?;
    println!("✓ Saved: architecture_with_patterns.dot");
    println!("  Generate SVG: dot -Tsvg architecture_with_patterns.dot > architecture.svg");

    let dot_with_positions = to_dot_with_positions(&model)?;
    fs::write("architecture_positioned.dot", &dot_with_positions)?;
    println!("\n✓ Saved: architecture_positioned.dot");
    println!("  Generate SVG: neato -n -Tsvg architecture_positioned.dot > positioned.svg");

    println!("\n📊 Color Legend:");
    println!("  🟢 Green  (#88dd88) - Healthy (severity = 0.0)");
    println!("  ⚪ Gray   (#cccccc) - Isolated (severity > 0.0, < 0.3)");
    println!("  🟡 Yellow (#ffcc44) - Warning (severity ≥ 0.3, < 0.7)");
    println!("  🟠 Orange (#ff8844) - Problem (severity ≥ 0.7, < 1.0)");
    println!("  🔴 Red    (#ff4444) - Critical (severity ≥ 1.0)");

    println!("\n✓ Visualization complete!");
    Ok(())
}
