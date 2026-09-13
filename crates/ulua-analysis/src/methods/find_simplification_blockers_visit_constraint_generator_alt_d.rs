use crate::{
  records::{
    find_simplification_blockers::FindSimplificationBlockers,
    pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.found = true;
    false
  }
}
