use crate::{OptaModel, Result};
use burn::tensor::backend::Backend;

pub struct OptaOptimizer<B: Backend> {
    iterations: usize,
    learning_rate: f32,
    _phantom: std::marker::PhantomData<B>,
}

impl<B: Backend> OptaOptimizer<B> {
    pub fn new(iterations: usize, learning_rate: f32) -> Self {
        Self {
            iterations,
            learning_rate,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn optimize_layout(&self, _model: &mut OptaModel<B>) -> Result<()> {
        Ok(())
    }
}

impl<B: Backend> Default for OptaOptimizer<B> {
    fn default() -> Self {
        Self::new(100, 0.1)
    }
}
