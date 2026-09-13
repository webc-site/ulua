use crate::records::{cfg_builder::CfgBuilder, symbol::Symbol};

impl CfgBuilder {
  pub fn next_version_index(&mut self, sym: Symbol) -> usize {
    if !self.version_counter.contains(&sym) {
      *self.version_counter.get_or_insert(sym) = 0;
      return 0;
    }

    let ref_mut = self.version_counter.find_mut(&sym).unwrap();
    *ref_mut += 1;
    *ref_mut
  }
}
