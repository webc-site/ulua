use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::{records::module_id_hash::ModuleIdHash, type_aliases::module_id::ModuleId};

impl ModuleIdHash {
  #[inline]
  pub fn shared_code_allocator_module_id_hash_operator_call(&self, module_id: &ModuleId) -> usize {
    let mut hasher = DefaultHasher::new();
    module_id.hash(&mut hasher);
    hasher.finish() as usize
  }
}
