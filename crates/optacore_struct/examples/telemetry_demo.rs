use burn::backend::ndarray::NdArray;
use optacore_struct::{
    AntiPatternConfig, OptaOptimizer, detect_anti_patterns_with_telemetry, parse_c4_dsl,
    telemetry::TelemetryLogger,
};
use std::path::PathBuf;

type Backend = NdArray<f32>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OptaCore Telemetry Demo");
    println!("=======================\n");

    let log_path = PathBuf::from("telemetry_demo.jsonl");
    let logger = TelemetryLogger::default().with_file(log_path.clone())?;

    let dsl = r#"
container API "API Gateway" "Node.js"
container AuthService "Authentication Service" "Java"
container UserDB "User Database" "PostgreSQL"
container CacheService "Cache Service" "Redis"
container NotificationService "Notification Service" "Python"

API -> AuthService "authenticates"
API -> CacheService "caches"
AuthService -> UserDB "queries"
AuthService -> CacheService "sessions"
NotificationService -> API "polls"
CacheService -> UserDB "reads"
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

    println!("Running force-directed layout optimization...");
    let optimizer = OptaOptimizer::<Backend>::new(100, 0.15);
    optimizer.optimize_layout_with_telemetry(&mut model, Some(&logger))?;
    println!("✓ Layout optimized\n");

    println!("Detecting anti-patterns...");
    let config = AntiPatternConfig::default();
    let patterns = detect_anti_patterns_with_telemetry(&model, &config, Some(&logger))?;

    if patterns.is_empty() {
        println!("✓ No anti-patterns detected!");
    } else {
        println!("⚠ Found {} anti-pattern(s):", patterns.len());
        for pattern in &patterns {
            println!("  - {:?}", pattern);
        }
    }

    println!("\n📊 Telemetry Events:");
    println!("Events logged to: {}", log_path.display());

    let content = std::fs::read_to_string(&log_path)?;
    let event_count = content.lines().count();
    println!("Total events: {}", event_count);

    println!("\nSample events:");
    for (i, line) in content.lines().enumerate() {
        if i < 3 {
            if let Ok(event) = serde_json::from_str::<serde_json::Value>(line) {
                println!(
                    "  {}. {} - {:?}",
                    i + 1,
                    event["event_type"].as_str().unwrap_or("unknown"),
                    event["payload"]
                );
            }
        }
    }

    println!("\n✓ Demo complete!");
    Ok(())
}
