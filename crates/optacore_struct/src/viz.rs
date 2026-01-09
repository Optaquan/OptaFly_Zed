use crate::{AntiPattern, AntiPatternConfig, OptaModel, detect_anti_patterns};
use burn::tensor::backend::Backend;
use std::collections::HashMap;

/// Export model to Graphviz DOT format with severity-based coloring
pub fn to_dot<B: Backend>(
    model: &OptaModel<B>,
    config: Option<&AntiPatternConfig>,
) -> crate::Result<String> {
    let mut dot = String::from("digraph Architecture {\n");
    dot.push_str("  rankdir=TB;\n");
    dot.push_str("  node [shape=box, style=filled];\n");
    dot.push_str("  edge [penwidth=1.5];\n\n");

    let patterns = if let Some(cfg) = config {
        detect_anti_patterns(model, cfg)?
    } else {
        Vec::new()
    };

    let severity_map = build_severity_map(&patterns);

    for node in &model.nodes {
        let severity = severity_map.get(&node.id).copied().unwrap_or(0.0);
        let color = severity_to_color(severity);
        let label = format!("{}", node.name);

        dot.push_str(&format!(
            "  \"{}\" [label=\"{}\", fillcolor=\"{}\"];\n",
            node.id, label, color
        ));

        if let Some((x, y)) = node.get_position() {
            dot.push_str(&format!("  // Position: ({:.2}, {:.2})\n", x, y));
        }
    }

    dot.push_str("\n");

    for edge in &model.edges {
        let label = edge.label.as_deref().unwrap_or("");
        let weight = edge.weight;

        let penwidth = 1.0 + (weight - 1.0) * 0.5;

        dot.push_str(&format!(
            "  \"{}\" -> \"{}\" [label=\"{}\", penwidth={}];\n",
            edge.from, edge.to, label, penwidth
        ));
    }

    if !patterns.is_empty() {
        dot.push_str("\n  // Anti-patterns detected:\n");
        for pattern in &patterns {
            match pattern {
                AntiPattern::Cycle { nodes } => {
                    dot.push_str(&format!("  // Cycle: {:?}\n", nodes));
                }
                AntiPattern::Bottleneck {
                    node_id,
                    in_degree,
                    severity,
                } => {
                    dot.push_str(&format!(
                        "  // Bottleneck: {} (in_degree={}, severity={:.2})\n",
                        node_id, in_degree, severity
                    ));
                }
                AntiPattern::IsolatedComponent { node_id } => {
                    dot.push_str(&format!("  // Isolated: {}\n", node_id));
                }
                AntiPattern::OverCoupling {
                    node_id,
                    out_degree,
                    severity,
                } => {
                    dot.push_str(&format!(
                        "  // Over-coupled: {} (out_degree={}, severity={:.2})\n",
                        node_id, out_degree, severity
                    ));
                }
            }
        }
    }

    dot.push_str("}\n");
    Ok(dot)
}

/// Export model with explicit positions (for neato -n layout)
pub fn to_dot_with_positions<B: Backend>(model: &OptaModel<B>) -> crate::Result<String> {
    let mut dot = String::from("graph Architecture {\n");
    dot.push_str("  layout=neato;\n");
    dot.push_str("  node [shape=box, style=filled, fixedsize=true, width=1.5, height=0.8];\n\n");

    for node in &model.nodes {
        if let Some((x, y)) = node.get_position() {
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", pos=\"{:.2},{:.2}!\"];\n",
                node.id, node.name, x, y
            ));
        } else {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", node.id, node.name));
        }
    }

    dot.push_str("\n");

    for edge in &model.edges {
        let label = edge.label.as_deref().unwrap_or("");
        dot.push_str(&format!(
            "  \"{}\" -- \"{}\" [label=\"{}\"];\n",
            edge.from, edge.to, label
        ));
    }

    dot.push_str("}\n");
    Ok(dot)
}

fn build_severity_map(patterns: &[AntiPattern]) -> HashMap<String, f32> {
    let mut severity_map: HashMap<String, f32> = HashMap::new();

    for pattern in patterns {
        match pattern {
            AntiPattern::Cycle { nodes } => {
                for node_id in nodes {
                    let current = severity_map.entry(node_id.clone()).or_insert(0.0);
                    *current = (*current).max(1.0);
                }
            }
            AntiPattern::Bottleneck {
                node_id, severity, ..
            } => {
                let current = severity_map.entry(node_id.clone()).or_insert(0.0);
                *current = (*current).max(*severity);
            }
            AntiPattern::IsolatedComponent { node_id } => {
                let current = severity_map.entry(node_id.clone()).or_insert(0.0);
                *current = (*current).max(0.3);
            }
            AntiPattern::OverCoupling {
                node_id, severity, ..
            } => {
                let current = severity_map.entry(node_id.clone()).or_insert(0.0);
                *current = (*current).max(*severity);
            }
        }
    }

    severity_map
}

fn severity_to_color(severity: f32) -> String {
    if severity >= 1.0 {
        "#ff4444".to_string()
    } else if severity >= 0.7 {
        "#ff8844".to_string()
    } else if severity >= 0.3 {
        "#ffcc44".to_string()
    } else if severity > 0.0 {
        "#cccccc".to_string()
    } else {
        "#88dd88".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeType, OptaNode, model::OptaEdge};
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[test]
    fn test_to_dot_empty_model() {
        let model = OptaModel::<TestBackend>::new();
        let dot = to_dot(&model, None).unwrap();
        assert!(dot.contains("digraph Architecture"));
    }

    #[test]
    fn test_to_dot_simple_model() {
        let mut model = OptaModel::<TestBackend>::new();
        model.add_node(OptaNode::new(
            "api".to_string(),
            "API".to_string(),
            NodeType::Container,
        ));
        model.add_node(OptaNode::new(
            "db".to_string(),
            "Database".to_string(),
            NodeType::Container,
        ));
        model.add_edge(OptaEdge::new("api".to_string(), "db".to_string()));

        let dot = to_dot(&model, None).unwrap();
        assert!(dot.contains("\"api\""));
        assert!(dot.contains("\"db\""));
        assert!(dot.contains("\"api\" -> \"db\""));
    }

    #[test]
    fn test_severity_coloring() {
        assert_eq!(severity_to_color(0.0), "#88dd88");
        assert_eq!(severity_to_color(0.2), "#cccccc");
        assert_eq!(severity_to_color(0.5), "#ffcc44");
        assert_eq!(severity_to_color(0.8), "#ff8844");
        assert_eq!(severity_to_color(1.5), "#ff4444");
    }
}
