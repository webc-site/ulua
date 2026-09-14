use crate::{
  records::{blocked_type::BlockedType, find_simplification_blockers::FindSimplificationBlockers},
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
    self.found = true;
    false
  }
}
