use burn::backend::ndarray::NdArray;
use optacore_struct::{
    AntiPatternConfig, NodeType, OptaModel, OptaNode, OptaOptimizer, model::OptaEdge, parse_c4_dsl,
    to_dot, to_dot_with_positions,
};
use std::fs;

type Backend = NdArray<f32>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OptaCore Enhanced Visualization Demo");
    println!("=====================================\n");

    println!("Example 1: Architecture with Bottleneck (API Gateway Pattern)");
    println!("--------------------------------------------------------------");
    demo_bottleneck_pattern()?;

    println!("\nExample 2: Cyclic Dependencies (Anti-Pattern)");
    println!("----------------------------------------------");
    demo_cycle_pattern()?;

    println!("\nExample 3: Mixed Node Types (C4 Shapes)");
    println!("----------------------------------------");
    demo_node_types()?;

    println!("\n✅ All visualizations generated!");
    println!("\n📊 To view:");
    println!("  dot -Tsvg example1_bottleneck.dot > example1.svg");
    println!("  dot -Tsvg example2_cycle.dot > example2.svg");
    println!("  dot -Tsvg example3_types.dot > example3.svg");

    Ok(())
}

fn demo_bottleneck_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let dsl = r#"
container Frontend "Frontend App" "React"
container API "API Gateway" "Node.js"
container AuthService "Auth Service" "Java"
container UserDB "User Database" "PostgreSQL"
container CacheService "Cache" "Redis"
container NotificationService "Notifications" "Python"

Frontend -> API "requests"
API -> AuthService "authenticates"
API -> CacheService "caches"
API -> NotificationService "sends"
AuthService -> UserDB "queries"
AuthService -> CacheService "sessions"
"#;

    let mut model = parse_c4_dsl::<Backend>(dsl)?;
    model.build_adjacency_matrix();

    let optimizer = OptaOptimizer::<Backend>::new(100, 0.2);
    optimizer.optimize_layout(&mut model)?;

    let config = AntiPatternConfig {
        bottleneck_threshold: 3,
        over_coupling_threshold: 5,
        detect_isolated: true,
    };

    let dot = to_dot(&model, Some(&config))?;
    fs::write("example1_bottleneck.dot", &dot)?;

    println!("  ✓ API Gateway node should be RED (bottleneck)");
    println!("  ✓ Other nodes should be GREEN (healthy)");
    println!("  ✓ Node shape: component (Container type)");

    Ok(())
}

fn demo_cycle_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let dsl = r#"
container ServiceA "Service A" "Java"
container ServiceB "Service B" "Node.js"
container ServiceC "Service C" "Python"
container ServiceD "Service D" "Go"

ServiceA -> ServiceB "calls"
ServiceB -> ServiceC "depends on"
ServiceC -> ServiceA "notifies"
ServiceD -> ServiceA "monitors"
"#;

    let mut model = parse_c4_dsl::<Backend>(dsl)?;
    model.build_adjacency_matrix();

    let optimizer = OptaOptimizer::<Backend>::new(80, 0.15);
    optimizer.optimize_layout(&mut model)?;

    let config = AntiPatternConfig::default();
    let dot = to_dot(&model, Some(&config))?;
    fs::write("example2_cycle.dot", &dot)?;

    println!("  ✓ ServiceA, B, C nodes should be RED (in cycle)");
    println!("  ✓ ServiceD should be GREEN (not in cycle)");
    println!("  ✓ Cycle edges should be RED with penwidth=3");
    println!("  ✓ ServiceD -> ServiceA edge should be normal");

    Ok(())
}

fn demo_node_types() -> Result<(), Box<dyn std::error::Error>> {
    let mut model = OptaModel::<Backend>::new();

    model.add_node(OptaNode::new(
        "user".to_string(),
        "End User".to_string(),
        NodeType::Person,
    ));

    model.add_node(OptaNode::new(
        "system".to_string(),
        "E-Commerce System".to_string(),
        NodeType::System,
    ));

    model.add_node(OptaNode::new(
        "webapp".to_string(),
        "Web Application".to_string(),
        NodeType::Container,
    ));

    model.add_node(OptaNode::new(
        "api_component".to_string(),
        "API Controller".to_string(),
        NodeType::Component,
    ));

    model.add_node(OptaNode::new(
        "db_server".to_string(),
        "Deployment Server".to_string(),
        NodeType::System,
    ));

    model.add_edge(
        OptaEdge::new("user".to_string(), "system".to_string()).with_label("uses".to_string()),
    );
    model.add_edge(
        OptaEdge::new("system".to_string(), "webapp".to_string())
            .with_label("includes".to_string()),
    );
    model.add_edge(
        OptaEdge::new("webapp".to_string(), "api_component".to_string())
            .with_label("contains".to_string()),
    );
    model.add_edge(
        OptaEdge::new("api_component".to_string(), "db_server".to_string())
            .with_label("reads/writes".to_string()),
    );

    model.build_adjacency_matrix();

    let optimizer = OptaOptimizer::<Backend>::new(60, 0.18);
    optimizer.optimize_layout(&mut model)?;

    let dot = to_dot(&model, None)?;
    fs::write("example3_types.dot", &dot)?;

    println!("  ✓ Person: ellipse shape");
    println!("  ✓ System: box3d shape");
    println!("  ✓ Container: component shape");
    println!("  ✓ Component: box shape");
    println!("  ✓ System: box3d shape");
    println!("  ✓ All nodes GREEN (no anti-patterns)");

    Ok(())
}
