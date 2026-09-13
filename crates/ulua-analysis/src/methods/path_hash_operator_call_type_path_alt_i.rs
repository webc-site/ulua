use crate::records::{path::Path, path_hash::PathHash};

impl PathHash {
  pub fn operator_call_6(&self, path: &Path) -> usize {
    let mut hash: usize = 0;
    for component in &path.components {
      hash ^= self.operator_component(component);
    }
    hash
  }
}
