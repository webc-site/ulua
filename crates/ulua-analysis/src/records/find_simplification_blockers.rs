use crate::{
  records::{
    blocked_type::BlockedType, extern_type::ExternType, free_type::FreeType,
    iterative_type_visitor::IterativeTypeVisitor, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct FindSimplificationBlockers {
  pub base: IterativeTypeVisitor,
  pub found: bool,
}

impl FindSimplificationBlockers {
  pub fn find_simplification_blockers(&mut self) {
    self.base.visit_once = true;
  }

  pub fn visit(&mut self, _ty: TypeId) -> bool {
    !self.found
  }

  pub fn visit_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.found = true;
    false
  }

  pub fn visit_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
