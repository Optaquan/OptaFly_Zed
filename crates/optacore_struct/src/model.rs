use burn::tensor::{Tensor, backend::Backend};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    System,
    Container,
    Component,
    Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptaNode<B: Backend> {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub description: Option<String>,
    pub technology: Option<String>,
    #[serde(skip)]
    pub position: Option<Tensor<B, 1>>,
}

impl<B: Backend> OptaNode<B> {
    pub fn new(id: String, name: String, node_type: NodeType) -> Self {
        Self {
            id,
            name,
            node_type,
            description: None,
            technology: None,
            position: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_technology(mut self, technology: String) -> Self {
        self.technology = Some(technology);
        self
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let device = B::Device::default();
        self.position = Some(Tensor::<B, 1>::from_floats([x, y], &device));
    }

    pub fn get_position(&self) -> Option<(f32, f32)> {
        self.position.as_ref().map(|tensor| {
            let data = tensor.to_data();
            let values: Vec<f32> = data.to_vec().expect("Failed to convert tensor data");
            (values[0], values[1])
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptaEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub weight: f32,
}

impl OptaEdge {
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            label: None,
            weight: 1.0,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

#[derive(Debug)]
pub struct OptaModel<B: Backend> {
    pub nodes: Vec<OptaNode<B>>,
    pub edges: Vec<OptaEdge>,
    pub adjacency_matrix: Option<Tensor<B, 2>>,
}

impl<B: Backend> OptaModel<B> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency_matrix: None,
        }
    }

    pub fn add_node(&mut self, node: OptaNode<B>) {
        self.nodes.push(node);
        self.adjacency_matrix = None;
    }

    pub fn add_edge(&mut self, edge: OptaEdge) {
        self.edges.push(edge);
        self.adjacency_matrix = None;
    }

    pub fn build_adjacency_matrix(&mut self) {
        let node_count = self.nodes.len();
        if node_count == 0 {
            self.adjacency_matrix = None;
            return;
        }

        let device = B::Device::default();
        let mut adjacency_data = vec![vec![0.0f32; node_count]; node_count];

        let node_index_map: std::collections::HashMap<&str, usize> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| (node.id.as_str(), idx))
            .collect();

        for edge in &self.edges {
            if let (Some(&from_idx), Some(&to_idx)) = (
                node_index_map.get(edge.from.as_str()),
                node_index_map.get(edge.to.as_str()),
            ) {
                adjacency_data[from_idx][to_idx] = edge.weight;
            }
        }

        let flat_data: Vec<f32> = adjacency_data.into_iter().flatten().collect();
        let tensor_1d: Tensor<B, 1> = Tensor::from_floats(flat_data.as_slice(), &device);
        let tensor_2d: Tensor<B, 2> = tensor_1d.reshape([node_count, node_count]);

        self.adjacency_matrix = Some(tensor_2d);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn find_node(&self, id: &str) -> Option<&OptaNode<B>> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn find_node_mut(&mut self, id: &str) -> Option<&mut OptaNode<B>> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
}

impl<B: Backend> Default for OptaModel<B> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[test]
    fn test_node_creation() {
        let node = OptaNode::<TestBackend>::new(
            "api".to_string(),
            "API Gateway".to_string(),
            NodeType::Container,
        );
        assert_eq!(node.id, "api");
        assert_eq!(node.name, "API Gateway");
        assert_eq!(node.node_type, NodeType::Container);
    }

    #[test]
    fn test_edge_creation() {
        let edge = OptaEdge::new("api".to_string(), "db".to_string())
            .with_label("queries".to_string())
            .with_weight(2.0);
        assert_eq!(edge.from, "api");
        assert_eq!(edge.to, "db");
        assert_eq!(edge.label, Some("queries".to_string()));
        assert_eq!(edge.weight, 2.0);
    }

    #[test]
    fn test_model_building() {
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

        assert_eq!(model.node_count(), 2);
        assert_eq!(model.edge_count(), 1);

        model.build_adjacency_matrix();
        assert!(model.adjacency_matrix.is_some());
    }

    #[test]
    fn test_node_position() {
        let mut node = OptaNode::<TestBackend>::new(
            "test".to_string(),
            "Test Node".to_string(),
            NodeType::Component,
        );

        node.set_position(100.0, 200.0);
        let pos = node.get_position().unwrap();
        assert_eq!(pos, (100.0, 200.0));
    }
}
