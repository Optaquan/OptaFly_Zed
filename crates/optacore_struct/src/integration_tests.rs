/// Integration tests demonstrating complete OptaCore workflow:
/// DSL parsing → model building → layout optimization → anti-pattern detection

#[cfg(test)]
mod tests {
    use crate::{AntiPattern, OptaOptimizer, detect_anti_patterns, parse_c4_dsl};
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[test]
    fn test_complete_workflow_simple() {
        let dsl = r#"
container API "API Gateway" "Node.js"
container DB "Database" "PostgreSQL"
container Cache "Redis Cache"

API -> DB "queries"
API -> Cache "caches"
"#;

        let mut model = parse_c4_dsl::<TestBackend>(dsl).expect("Failed to parse DSL");
        assert_eq!(model.node_count(), 3);
        assert_eq!(model.edge_count(), 2);

        model.build_adjacency_matrix();
        assert!(model.adjacency_matrix.is_some());

        let optimizer = OptaOptimizer::<TestBackend>::new(50, 0.1);
        optimizer
            .optimize_layout(&mut model)
            .expect("Optimization failed");

        let api = model.find_node("API").expect("API node not found");
        assert!(
            api.get_position().is_some(),
            "API position not set after optimization"
        );

        let patterns = detect_anti_patterns(&model, &crate::AntiPatternConfig::default()).expect("Pattern detection failed");
        assert_eq!(patterns.len(), 0, "Well-formed architecture should have no anti-patterns");
    }

    #[test]
    fn test_complete_workflow_with_cycle() {
        let dsl = r#"
container ServiceA "Service A"
container ServiceB "Service B"
container ServiceC "Service C"

ServiceA -> ServiceB "calls"
ServiceB -> ServiceC "depends on"
ServiceC -> ServiceA "notifies"
"#;

        let mut model = parse_c4_dsl::<TestBackend>(dsl).expect("Failed to parse DSL");
        model.build_adjacency_matrix();

        let optimizer = OptaOptimizer::<TestBackend>::new(30, 0.15);
        optimizer
            .optimize_layout(&mut model)
            .expect("Optimization failed");

        for node in &model.nodes {
            assert!(
                node.get_position().is_some(),
                "Node {} missing position",
                node.id
            );
        }

        let patterns = detect_anti_patterns(&model, &crate::AntiPatternConfig::default()).expect("Pattern detection failed");
        assert!(!patterns.is_empty(), "Should detect cycle");

        let has_cycle = patterns
            .iter()
            .any(|p| matches!(p, AntiPattern::Cycle { .. }));
        assert!(has_cycle, "Cycle not detected");
    }

    #[test]
    fn test_complete_workflow_bottleneck_detection() {
        let dsl = r#"
container Database "Central Database"
container Service1 "Service 1"
container Service2 "Service 2"
container Service3 "Service 3"
container Service4 "Service 4"
container Service5 "Service 5"
container Service6 "Service 6"

Service1 -> Database "writes"
Service2 -> Database "reads"
Service3 -> Database "queries"
Service4 -> Database "updates"
Service5 -> Database "deletes"
Service6 -> Database "inserts"
"#;

        let mut model = parse_c4_dsl::<TestBackend>(dsl).expect("Failed to parse DSL");
        model.build_adjacency_matrix();

        let optimizer = OptaOptimizer::<TestBackend>::new(40, 0.12).with_area(2000.0);
        optimizer
            .optimize_layout(&mut model)
            .expect("Optimization failed");

        let patterns = detect_anti_patterns(&model, &crate::AntiPatternConfig::default()).expect("Pattern detection failed");

        let has_bottleneck = patterns.iter().any(|p| {
            matches!(p, AntiPattern::Bottleneck { node_id, in_degree, .. }
                if node_id == "Database" && *in_degree >= 5)
        });
        assert!(has_bottleneck, "Bottleneck not detected for Database");
    }

    #[test]
    fn test_complete_workflow_god_object() {
        let dsl = r#"
container GodService "God Service"
container Dep1 "Dependency 1"
container Dep2 "Dependency 2"
container Dep3 "Dependency 3"
container Dep4 "Dependency 4"
container Dep5 "Dependency 5"
container Dep6 "Dependency 6"
container Dep7 "Dependency 7"
container Dep8 "Dependency 8"

GodService -> Dep1 "uses"
GodService -> Dep2 "calls"
GodService -> Dep3 "depends"
GodService -> Dep4 "queries"
GodService -> Dep5 "updates"
GodService -> Dep6 "manages"
GodService -> Dep7 "controls"
GodService -> Dep8 "monitors"
"#;

        let mut model = parse_c4_dsl::<TestBackend>(dsl).expect("Failed to parse DSL");
        model.build_adjacency_matrix();

        let optimizer = OptaOptimizer::<TestBackend>::new(50, 0.1);
        optimizer
            .optimize_layout(&mut model)
            .expect("Optimization failed");

        let patterns = detect_anti_patterns(&model, &crate::AntiPatternConfig::default()).expect("Pattern detection failed");

        let has_over_coupling = patterns.iter().any(|p| {
            matches!(p, AntiPattern::OverCoupling { node_id, out_degree, .. }
                if node_id == "GodService" && *out_degree >= 7)
        });
        assert!(has_over_coupling, "Over-coupling (God Object) not detected");
    }

    #[test]
    fn test_complete_workflow_positions_change() {
        let dsl = r#"
container A "Node A"
container B "Node B"
A -> B "connects"
"#;

        let mut model = parse_c4_dsl::<TestBackend>(dsl).expect("Failed to parse DSL");
        model.build_adjacency_matrix();

        let node_a_before = model.find_node("A").unwrap().get_position();
        assert!(
            node_a_before.is_none(),
            "Position should be None before optimization"
        );

        let optimizer = OptaOptimizer::<TestBackend>::new(20, 0.2);
        optimizer
            .optimize_layout(&mut model)
            .expect("Optimization failed");

        let node_a_after = model.find_node("A").unwrap().get_position();
        let node_b_after = model.find_node("B").unwrap().get_position();

        assert!(
            node_a_after.is_some(),
            "Position should be set after optimization"
        );
        assert!(
            node_b_after.is_some(),
            "Position should be set after optimization"
        );

        let (ax, ay) = node_a_after.unwrap();
        let (bx, by) = node_b_after.unwrap();

        assert!(ax >= 0.0 && ax <= 100.0, "X position out of bounds: {}", ax);
        assert!(ay >= 0.0 && ay <= 100.0, "Y position out of bounds: {}", ay);
        assert!(bx >= 0.0 && bx <= 100.0, "X position out of bounds: {}", bx);
        assert!(by >= 0.0 && by <= 100.0, "Y position out of bounds: {}", by);
    }
}
