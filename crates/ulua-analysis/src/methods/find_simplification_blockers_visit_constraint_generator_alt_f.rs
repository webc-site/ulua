use crate::{
  records::{extern_type::ExternType, find_simplification_blockers::FindSimplificationBlockers},
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
