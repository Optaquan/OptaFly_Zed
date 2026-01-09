use crate::{OptaModel, Result};
use burn::tensor::backend::Backend;

pub fn parse_c4_dsl<B: Backend>(_input: &str) -> Result<OptaModel<B>> {
    Ok(OptaModel::new())
}
