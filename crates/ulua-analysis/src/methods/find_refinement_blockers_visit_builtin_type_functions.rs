use crate::{
  records::{
    blocked_type::BlockedType, extern_type::ExternType,
    find_refinement_blockers::FindRefinementBlockers, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindRefinementBlockers {
  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _blocked: &BlockedType) -> bool {
    self.found.insert(ty);
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _pending: &PendingExpansionType,
  ) -> bool {
    self.found.insert(ty);
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern: &ExternType) -> bool {
    false
  }
}
