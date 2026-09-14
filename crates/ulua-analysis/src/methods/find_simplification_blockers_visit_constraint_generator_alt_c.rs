use crate::{
  records::{find_simplification_blockers::FindSimplificationBlockers, free_type::FreeType},
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
    self.found = true;
    false
  }
}
