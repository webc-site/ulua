use crate::{
  records::{
    find_simplification_blockers::FindSimplificationBlockers, function_type::FunctionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, _ftv: &FunctionType) -> bool {
    false
  }
}
