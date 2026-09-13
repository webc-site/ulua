use crate::{
  records::{blocked_type::BlockedType, find_refinement_blockers::FindRefinementBlockers},
  type_aliases::type_id::TypeId,
};

impl FindRefinementBlockers {
  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _blocked: &BlockedType) -> bool {
    self.found.insert(ty);
    false
  }
}
