use crate::{
  records::{
    find_refinement_blockers::FindRefinementBlockers, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindRefinementBlockers {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _pending: &PendingExpansionType,
  ) -> bool {
    self.found.insert(ty);
    false
  }
}
