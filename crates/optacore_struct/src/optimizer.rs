use crate::{OptaModel, Result};
use burn::tensor::{Tensor, backend::Backend};
use rand::Rng;

/// Force-directed layout optimizer using Fruchterman-Reingold algorithm
///
/// Positions nodes in 2D space by simulating physical forces:
/// - Attractive forces between connected nodes (edges)
/// - Repulsive forces between all node pairs
///
/// Uses gradient descent with Burn tensors for performance optimization.
pub struct OptaOptimizer<B: Backend> {
    iterations: usize,
    learning_rate: f32,
    area: f32,
    k: f32,
    _phantom: std::marker::PhantomData<B>,
}

impl<B: Backend> OptaOptimizer<B> {
    pub fn new(iterations: usize, learning_rate: f32) -> Self {
        Self {
            iterations,
            learning_rate,
            area: 1000.0,
            k: 50.0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_area(mut self, area: f32) -> Self {
        self.area = area;
        self.k = (area / self.iterations as f32).sqrt();
        self
    }

    /// Optimize node positions using force-directed layout
    ///
    /// Returns error if model has no nodes or adjacency matrix not built
    pub fn optimize_layout(&self, model: &mut OptaModel<B>) -> Result<()> {
        let node_count = model.node_count();
        if node_count == 0 {
            return Err(anyhow::anyhow!("Cannot optimize empty model"));
        }

        if model.adjacency_matrix.is_none() {
            return Err(anyhow::anyhow!(
                "Adjacency matrix not built. Call build_adjacency_matrix() first"
            ));
        }

        let device = B::Device::default();
        let mut positions = self.initialize_positions(node_count, &device);

        for iteration in 0..self.iterations {
            let cooling = 1.0 - (iteration as f32 / self.iterations as f32);
            let temperature = self.learning_rate * cooling;

            let displacement =
                self.compute_forces(&positions, model.adjacency_matrix.as_ref().unwrap());

            positions = positions.clone() + displacement * temperature;
            positions = self.clamp_positions(positions);
        }

        self.apply_positions_to_model(model, positions)?;

        Ok(())
    }

    fn initialize_positions(&self, count: usize, device: &B::Device) -> Tensor<B, 2> {
        let mut rng = rand::rng();
        let mut positions = Vec::with_capacity(count * 2);

        for _ in 0..count {
            let x = rng.random_range(0.0..self.area.sqrt());
            let y = rng.random_range(0.0..self.area.sqrt());
            positions.push(x);
            positions.push(y);
        }

        Tensor::<B, 1>::from_floats(positions.as_slice(), device).reshape([count, 2])
    }

    fn compute_forces(&self, positions: &Tensor<B, 2>, adjacency: &Tensor<B, 2>) -> Tensor<B, 2> {
        let node_count = positions.dims()[0];
        let device = positions.device();

        let positions_data = positions.to_data();
        let positions_vec: Vec<f32> = positions_data
            .to_vec()
            .expect("Failed to convert positions");

        let adjacency_data = adjacency.to_data();
        let adjacency_vec: Vec<f32> = adjacency_data
            .to_vec()
            .expect("Failed to convert adjacency");

        let mut force_values = vec![0.0f32; node_count * 2];

        for i in 0..node_count {
            let xi = positions_vec[i * 2];
            let yi = positions_vec[i * 2 + 1];

            for j in 0..node_count {
                if i == j {
                    continue;
                }

                let xj = positions_vec[j * 2];
                let yj = positions_vec[j * 2 + 1];

                let dx = xi - xj;
                let dy = yi - yj;
                let distance_sq = dx * dx + dy * dy + 0.01;
                let distance = distance_sq.sqrt();

                let repulsive_force = (self.k * self.k) / distance_sq;
                force_values[i * 2] += (dx / distance) * repulsive_force;
                force_values[i * 2 + 1] += (dy / distance) * repulsive_force;

                let edge_weight = adjacency_vec[i * node_count + j];
                if edge_weight > 0.0 {
                    let attractive_force = (distance * distance) / self.k * edge_weight;
                    force_values[i * 2] -= (dx / distance) * attractive_force;
                    force_values[i * 2 + 1] -= (dy / distance) * attractive_force;
                }
            }
        }

        Tensor::<B, 1>::from_floats(force_values.as_slice(), &device).reshape([node_count, 2])
    }

    fn clamp_positions(&self, positions: Tensor<B, 2>) -> Tensor<B, 2> {
        let max_val = self.area.sqrt();
        positions.clamp(0.0, max_val)
    }

    fn apply_positions_to_model(
        &self,
        model: &mut OptaModel<B>,
        positions: Tensor<B, 2>,
    ) -> Result<()> {
        let positions_data = positions.to_data();
        let values: Vec<f32> = positions_data
            .to_vec()
            .expect("Failed to convert tensor to vector");

        for (idx, node) in model.nodes.iter_mut().enumerate() {
            let x = values[idx * 2];
            let y = values[idx * 2 + 1];
            node.set_position(x, y);
        }

        Ok(())
    }
}

impl<B: Backend> Default for OptaOptimizer<B> {
    fn default() -> Self {
        Self::new(100, 0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeType, OptaNode, model::OptaEdge};
    use burn::backend::ndarray::NdArray;

    type TestBackend = NdArray<f32>;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = OptaOptimizer::<TestBackend>::new(50, 0.05);
        assert_eq!(optimizer.iterations, 50);
        assert_eq!(optimizer.learning_rate, 0.05);
    }

    #[test]
    fn test_optimize_empty_model() {
        let optimizer = OptaOptimizer::<TestBackend>::default();
        let mut model = OptaModel::new();

        let result = optimizer.optimize_layout(&mut model);
        assert!(result.is_err());
    }

    #[test]
    fn test_optimize_no_adjacency_matrix() {
        let optimizer = OptaOptimizer::<TestBackend>::default();
        let mut model = OptaModel::new();
        model.add_node(OptaNode::new(
            "a".to_string(),
            "A".to_string(),
            NodeType::Container,
        ));

        let result = optimizer.optimize_layout(&mut model);
        assert!(result.is_err());
    }

    #[test]
    fn test_optimize_simple_graph() {
        let optimizer = OptaOptimizer::<TestBackend>::new(10, 0.1);
        let mut model = OptaModel::new();

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
        model.add_edge(OptaEdge::new("a".to_string(), "b".to_string()));

        model.build_adjacency_matrix();

        let result = optimizer.optimize_layout(&mut model);
        assert!(result.is_ok());

        let pos_a = model.find_node("a").unwrap().get_position();
        let pos_b = model.find_node("b").unwrap().get_position();

        assert!(pos_a.is_some());
        assert!(pos_b.is_some());
    }
}
