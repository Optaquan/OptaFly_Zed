use crate::{OptaModel, Result};
use burn::tensor::backend::Backend;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AntiPattern {
    /// Circular dependency detected in architecture
    Cycle { nodes: Vec<String> },
    /// Single point of failure with many incoming dependencies
    Bottleneck {
        node_id: String,
        in_degree: usize,
        severity: f32,
    },
    /// Component with no connections (potentially orphaned)
    IsolatedComponent { node_id: String },
    /// Component with excessive outgoing dependencies (God Object pattern)
    OverCoupling {
        node_id: String,
        out_degree: usize,
        severity: f32,
    },
}

#[derive(Debug, Clone)]
pub struct AntiPatternConfig {
    pub bottleneck_threshold: usize,
    pub over_coupling_threshold: usize,
    pub detect_isolated: bool,
}

impl Default for AntiPatternConfig {
    fn default() -> Self {
        Self {
            bottleneck_threshold: 5,
            over_coupling_threshold: 7,
            detect_isolated: true,
        }
    }
}

/// Detect architectural anti-patterns in the model with custom configuration
pub fn detect_anti_patterns<B: Backend>(
    model: &OptaModel<B>,
    config: &AntiPatternConfig,
) -> Result<Vec<AntiPattern>> {
    let mut patterns = Vec::new();

    if model.node_count() == 0 {
        return Ok(patterns);
    }

    let adjacency = build_adjacency_map(model);

    patterns.extend(detect_cycles(model, &adjacency)?);
    patterns.extend(detect_bottlenecks(&adjacency, config.bottleneck_threshold));
    patterns.extend(detect_over_coupling(
        &adjacency,
        config.over_coupling_threshold,
    ));
    if config.detect_isolated {
        patterns.extend(detect_isolated_components(model, &adjacency));
    }

    Ok(patterns)
}

fn build_adjacency_map<B: Backend>(model: &OptaModel<B>) -> HashMap<String, Vec<String>> {
    let mut adjacency = HashMap::new();

    for node in &model.nodes {
        adjacency.insert(node.id.clone(), Vec::new());
    }

    for edge in &model.edges {
        adjacency
            .entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push(edge.to.clone());
    }

    adjacency
}

fn detect_cycles<B: Backend>(
    model: &OptaModel<B>,
    adjacency: &HashMap<String, Vec<String>>,
) -> Result<Vec<AntiPattern>> {
    let mut patterns = Vec::new();
    let mut color: HashMap<String, Color> = HashMap::new();
    let mut parent: HashMap<String, Option<String>> = HashMap::new();

    for node in &model.nodes {
        color.insert(node.id.clone(), Color::White);
        parent.insert(node.id.clone(), None);
    }

    for node in &model.nodes {
        if color[&node.id] == Color::White {
            if let Some(cycle) = dfs_visit(&node.id, adjacency, &mut color, &mut parent) {
                patterns.push(AntiPattern::Cycle { nodes: cycle });
            }
        }
    }

    Ok(patterns)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

fn dfs_visit(
    node_id: &str,
    adjacency: &HashMap<String, Vec<String>>,
    color: &mut HashMap<String, Color>,
    parent: &mut HashMap<String, Option<String>>,
) -> Option<Vec<String>> {
    color.insert(node_id.to_string(), Color::Gray);

    if let Some(neighbors) = adjacency.get(node_id) {
        for neighbor in neighbors {
            match color.get(neighbor) {
                Some(Color::White) => {
                    parent.insert(neighbor.clone(), Some(node_id.to_string()));
                    if let Some(cycle) = dfs_visit(neighbor, adjacency, color, parent) {
                        return Some(cycle);
                    }
                }
                Some(Color::Gray) => {
                    let cycle = reconstruct_cycle(neighbor, node_id, parent);
                    return Some(cycle);
                }
                _ => {}
            }
        }
    }

    color.insert(node_id.to_string(), Color::Black);
    None
}

fn reconstruct_cycle(
    cycle_start: &str,
    cycle_end: &str,
    parent: &HashMap<String, Option<String>>,
) -> Vec<String> {
    let mut cycle = vec![cycle_start.to_string()];
    let mut current = cycle_end;

    while current != cycle_start {
        cycle.push(current.to_string());
        if let Some(Some(p)) = parent.get(current) {
            current = p;
        } else {
            break;
        }
    }

    cycle.reverse();
    cycle
}

fn detect_bottlenecks(
    adjacency: &HashMap<String, Vec<String>>,
    threshold: usize,
) -> Vec<AntiPattern> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();

    for node_id in adjacency.keys() {
        in_degree.insert(node_id.clone(), 0);
    }

    for neighbors in adjacency.values() {
        for neighbor in neighbors {
            *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
        }
    }

    in_degree
        .into_iter()
        .filter(|(_, degree)| *degree >= threshold)
        .map(|(node_id, in_degree)| {
            let severity = (in_degree as f32) / (threshold as f32).max(1.0);
            AntiPattern::Bottleneck {
                node_id,
                in_degree,
                severity,
            }
        })
        .collect()
}

fn detect_over_coupling(
    adjacency: &HashMap<String, Vec<String>>,
    threshold: usize,
) -> Vec<AntiPattern> {
    adjacency
        .iter()
        .filter(|(_, neighbors)| neighbors.len() >= threshold)
        .map(|(node_id, neighbors)| {
            let out_degree = neighbors.len();
            let severity = (out_degree as f32) / (threshold as f32).max(1.0);
            AntiPattern::OverCoupling {
                node_id: node_id.clone(),
                out_degree,
                severity,
            }
        })
        .collect()
}

fn detect_isolated_components<B: Backend>(
    model: &OptaModel<B>,
    adjacency: &HashMap<String, Vec<String>>,
) -> Vec<AntiPattern> {
    let mut has_incoming: HashSet<String> = HashSet::new();

    for neighbors in adjacency.values() {
        for neighbor in neighbors {
            has_incoming.insert(neighbor.clone());
        }
    }

    model
        .nodes
        .iter()
        .filter(|node| {
            let has_outgoing = adjacency
                .get(&node.id)
                .map(|n| !n.is_empty())
                .unwrap_or(false);
            let has_incoming_edge = has_incoming.contains(&node.id);
            !has_outgoing && !has_incoming_edge
        })
        .map(|node| AntiPattern::IsolatedComponent {
            node_id: node.id.clone(),
        })
        .collect()
}


/// Detect anti-patterns with optional telemetry logging
#[cfg(feature = "telemetry")]
pub fn detect_anti_patterns_with_telemetry<B: Backend>(
    model: &OptaModel<B>,
    config: &AntiPatternConfig,
    logger: Option<&crate::telemetry::TelemetryLogger>,
) -> Result<Vec<AntiPattern>> {
    let patterns = detect_anti_patterns(model, config)?;

    if let Some(logger) = logger {
        for pattern in &patterns {
            match pattern {
                AntiPattern::Cycle { nodes } => {
                    logger.log_pattern_detected(
                        "cycle".to_string(),
                        1.0,
                        nodes.clone(),
                    )?;
                }
                AntiPattern::Bottleneck { node_id, severity, .. } => {
                    logger.log_pattern_detected(
                        "bottleneck".to_string(),
                        *severity,
                        vec![node_id.clone()],
                    )?;
                }
                AntiPattern::IsolatedComponent { node_id } => {
                    logger.log_pattern_detected(
                        "isolated".to_string(),
                        0.5,
                        vec![node_id.clone()],
                    )?;
                }
                AntiPattern::OverCoupling { node_id, severity, .. } => {
                    logger.log_pattern_detected(
                        "over_coupling".to_string(),
                        *severity,
                        vec![node_id.clone()],
                    )?;
                }
            }
        }
    }

    Ok(patterns)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeType, OptaNode, model::OptaEdge};
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[test]
    fn test_no_patterns_empty_model() {
        let model = OptaModel::<TestBackend>::new();
        let config = AntiPatternConfig::default();
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert_eq!(patterns.len(), 0);
    }

    #[test]
    fn test_detect_cycle() {
        let mut model = OptaModel::<TestBackend>::new();
        model.add_node(OptaNode::new(
            "a".to_string(),
            "A".to_string(),
            NodeType::Container,
        ));
        model.add_node(OptaNode::new(
            "b".to_string(),
            "B".to_string(),
            NodeType::Container,
        ));
        model.add_node(OptaNode::new(
            "c".to_string(),
            "C".to_string(),
            NodeType::Container,
        ));

        model.add_edge(OptaEdge::new("a".to_string(), "b".to_string()));
        model.add_edge(OptaEdge::new("b".to_string(), "c".to_string()));
        model.add_edge(OptaEdge::new("c".to_string(), "a".to_string()));

        let config = AntiPatternConfig::default();
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert!(!patterns.is_empty());
        assert!(
            patterns
                .iter()
                .any(|p| matches!(p, AntiPattern::Cycle { .. }))
        );
    }

    #[test]
    fn test_detect_bottleneck() {
        let mut model = OptaModel::<TestBackend>::new();
        model.add_node(OptaNode::new(
            "hub".to_string(),
            "Hub".to_string(),
            NodeType::Container,
        ));

        for i in 0..6 {
            let id = format!("node{}", i);
            model.add_node(OptaNode::new(
                id.clone(),
                format!("Node {}", i),
                NodeType::Container,
            ));
            model.add_edge(OptaEdge::new(id, "hub".to_string()));
        }

        let config = AntiPatternConfig::default();
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert!(patterns.iter().any(|p| matches!(p, AntiPattern::Bottleneck { node_id, in_degree, severity } if node_id == "hub" && *in_degree >= 5 && *severity >= 1.0)));
    }

    #[test]
    fn test_detect_over_coupling() {
        let mut model = OptaModel::<TestBackend>::new();
        model.add_node(OptaNode::new(
            "god".to_string(),
            "God Object".to_string(),
            NodeType::Container,
        ));

        for i in 0..8 {
            let id = format!("dep{}", i);
            model.add_node(OptaNode::new(
                id.clone(),
                format!("Dependency {}", i),
                NodeType::Container,
            ));
            model.add_edge(OptaEdge::new("god".to_string(), id));
        }

        let config = AntiPatternConfig::default();
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert!(patterns.iter().any(|p| matches!(p, AntiPattern::OverCoupling { node_id, out_degree, severity } if node_id == "god" && *out_degree >= 7 && *severity >= 1.0)));
    }

    #[test]
    fn test_detect_isolated_component() {
        let mut model = OptaModel::<TestBackend>::new();
        model.add_node(OptaNode::new(
            "connected1".to_string(),
            "Connected 1".to_string(),
            NodeType::Container,
        ));
        model.add_node(OptaNode::new(
            "connected2".to_string(),
            "Connected 2".to_string(),
            NodeType::Container,
        ));
        model.add_node(OptaNode::new(
            "isolated".to_string(),
            "Isolated".to_string(),
            NodeType::Container,
        ));

        model.add_edge(OptaEdge::new(
            "connected1".to_string(),
            "connected2".to_string(),
        ));

        let config = AntiPatternConfig::default();
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert!(patterns.iter().any(
            |p| matches!(p, AntiPattern::IsolatedComponent { node_id } if node_id == "isolated")
        ));
    }

    #[test]
    fn test_configurable_thresholds() {
        let mut model = OptaModel::<TestBackend>::new();
        model.add_node(OptaNode::new(
            "hub".to_string(),
            "Hub".to_string(),
            NodeType::Container,
        ));

        for i in 0..3 {
            let id = format!("node{}", i);
            model.add_node(OptaNode::new(
                id.clone(),
                format!("Node {}", i),
                NodeType::Container,
            ));
            model.add_edge(OptaEdge::new(id, "hub".to_string()));
        }

        let mut config = AntiPatternConfig::default();
        config.bottleneck_threshold = 4;
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert!(
            patterns
                .iter()
                .all(|p| !matches!(p, AntiPattern::Bottleneck { .. }))
        );

        config.bottleneck_threshold = 3;
        let patterns = detect_anti_patterns(&model, &config).unwrap();
        assert!(
            patterns
                .iter()
                .any(|p| matches!(p, AntiPattern::Bottleneck { .. }))
        );
    }
}
