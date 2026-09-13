use alloc::{string::String, vec::Vec};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequireAlias {
  pub(crate) alias: String,
  pub(crate) tags: Vec<String>,
}
