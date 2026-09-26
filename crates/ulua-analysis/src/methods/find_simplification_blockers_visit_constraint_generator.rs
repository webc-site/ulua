use crate::{
  records::{
    blocked_type::BlockedType, extern_type::ExternType,
    find_simplification_blockers::FindSimplificationBlockers, free_type::FreeType,
    function_type::FunctionType, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }

  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, _ftv: &FunctionType) -> bool {
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
