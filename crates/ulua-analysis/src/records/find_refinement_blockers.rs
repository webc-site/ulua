use alloc::string::String;
use core::ptr;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    blocked_type::BlockedType, extern_type::ExternType,
    pending_expansion_type::PendingExpansionType, type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct FindRefinementBlockers {
  pub base: TypeOnceVisitor,
  pub found: DenseHashSet<TypeId>,
}

impl FindRefinementBlockers {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("FindRefinementBlockers"), true),
      found: DenseHashSet::new(ptr::null()),
    }
  }

  pub fn visit_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
    self.found.insert(ty);
    false
  }

  pub fn visit_pending_expansion_type(&mut self, ty: TypeId, _petv: &PendingExpansionType) -> bool {
    self.found.insert(ty);
    false
  }

  pub fn visit_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}

impl Default for FindRefinementBlockers {
  fn default() -> Self {
    Self::new()
  }
}
