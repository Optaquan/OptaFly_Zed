use crate::{NodeType, OptaModel, OptaNode, Result, model::OptaEdge};
use burn::tensor::backend::Backend;
use regex::Regex;

/// Parse C4 DSL into OptaModel (MVP regex-based implementation)
///
/// **⚠️ Current Limitations:**
/// - **Regex-based**: Fragile for complex DSL, no proper tokenization
/// - **No nesting**: All nodes are flat (ignores `system { container { ... } }` hierarchy)
/// - **Limited error reporting**: No line numbers or context in errors
/// - **Missing features**: softwareSystem, deploymentNode, tags, descriptions, comments
/// - **No multi-line**: Descriptions and whitespace handling is basic
///
/// **Recommended Production Alternative:**
/// Use the official Structurizr DSL parser:
/// - **Repository**: https://github.com/structurizr/java/tree/master/structurizr-dsl
/// - **Integration options**:
///   1. JNI bridge to Java parser (most robust)
///   2. External process: Java → JSON → Rust (simpler, still reliable)
///   3. Port to Rust with nom/pest (full control, more work)
///
/// **Supported Syntax (Basic):**
/// ```text
/// system MySystem "System Name"
/// container API "API Gateway" "Node.js"
/// component Auth "Authentication" "JWT"
/// person User "Customer"
///
/// API -> DB "queries"
/// User -> API "makes requests"
/// ```
pub fn parse_c4_dsl<B: Backend>(input: &str) -> Result<OptaModel<B>> {
    let mut model = OptaModel::new();

    let container_re = Regex::new(r#"(?m)^\s*container\s+(\w+)\s+"([^"]+)"(?:\s+"([^"]+)")?"#)?;
    let component_re = Regex::new(r#"(?m)^\s*component\s+(\w+)\s+"([^"]+)"(?:\s+"([^"]+)")?"#)?;
    let person_re = Regex::new(r#"(?m)^\s*person\s+(\w+)\s+"([^"]+)""#)?;
    let system_re = Regex::new(r#"(?m)^\s*system\s+(\w+)\s+"([^"]+)""#)?;
    let relationship_re = Regex::new(r#"(?m)^\s*(\w+)\s*->\s*(\w+)(?:\s+"([^"]+)")?"#)?;

    for cap in container_re.captures_iter(input) {
        let id = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing container ID"))?;
        let name = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing container name"))?;
        let technology = cap.get(3).map(|m| m.as_str().to_string());

        let mut node = OptaNode::new(id, name, NodeType::Container);
        if let Some(tech) = technology {
            node = node.with_technology(tech);
        }
        model.add_node(node);
    }

    for cap in component_re.captures_iter(input) {
        let id = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing component ID"))?;
        let name = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing component name"))?;
        let technology = cap.get(3).map(|m| m.as_str().to_string());

        let mut node = OptaNode::new(id, name, NodeType::Component);
        if let Some(tech) = technology {
            node = node.with_technology(tech);
        }
        model.add_node(node);
    }

    for cap in person_re.captures_iter(input) {
        let id = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing person ID"))?;
        let name = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing person name"))?;

        let node = OptaNode::new(id, name, NodeType::Person);
        model.add_node(node);
    }

    for cap in system_re.captures_iter(input) {
        let id = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing system ID"))?;
        let name = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing system name"))?;

        let node = OptaNode::new(id, name, NodeType::System);
        model.add_node(node);
    }

    for cap in relationship_re.captures_iter(input) {
        let from = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing relationship source"))?;
        let to = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing relationship target"))?;
        let label = cap.get(3).map(|m| m.as_str().to_string());

        let mut edge = OptaEdge::new(from, to);
        if let Some(l) = label {
            edge = edge.with_label(l);
        }
        model.add_edge(edge);
    }

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[test]
    fn test_parse_empty() {
        let result = parse_c4_dsl::<TestBackend>("");
        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.node_count(), 0);
    }

    #[test]
    fn test_parse_single_container() {
        let dsl = r#"container API "API Gateway" "Node.js""#;
        let result = parse_c4_dsl::<TestBackend>(dsl);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(model.node_count(), 1);

        let node = model.find_node("API").unwrap();
        assert_eq!(node.name, "API Gateway");
        assert_eq!(node.node_type, NodeType::Container);
        assert_eq!(node.technology, Some("Node.js".to_string()));
    }

    #[test]
    fn test_parse_relationship() {
        let dsl = r#"
container API "API Gateway"
container DB "Database"
API -> DB "queries"
"#;
        let result = parse_c4_dsl::<TestBackend>(dsl);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(model.node_count(), 2);
        assert_eq!(model.edge_count(), 1);

        let edge = &model.edges[0];
        assert_eq!(edge.from, "API");
        assert_eq!(edge.to, "DB");
        assert_eq!(edge.label, Some("queries".to_string()));
    }

    #[test]
    fn test_parse_full_system() {
        let dsl = r#"
system ECommerce "E-Commerce Platform"

container WebApp "Web Application" "React"
container API "API Gateway" "Node.js"
container DB "Database" "PostgreSQL"

component Auth "Authentication" "JWT"
component Cart "Shopping Cart"

person User "Customer"

User -> WebApp "browses"
WebApp -> API "makes requests"
API -> DB "queries"
API -> Auth "validates"
Auth -> DB "reads credentials"
"#;
        let result = parse_c4_dsl::<TestBackend>(dsl);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(model.node_count(), 7);
        assert_eq!(model.edge_count(), 5);

        assert!(model.find_node("ECommerce").is_some());
        assert!(model.find_node("User").is_some());
        assert!(model.find_node("Auth").is_some());
    }
}
