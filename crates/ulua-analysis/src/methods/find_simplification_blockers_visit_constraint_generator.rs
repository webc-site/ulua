use crate::{
  records::find_simplification_blockers::FindSimplificationBlockers, type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }
}
