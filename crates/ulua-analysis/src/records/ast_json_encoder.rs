use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone)]
pub struct AstJsonEncoder {
  pub(crate) chunks: Vec<String>,
  pub(crate) comma: bool,
}
