use crate::records::{
  primitive_type::PrimitiveType, scope::Scope, subtyping::Subtyping,
  subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  table_type::TableType,
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    _sub_table: &TableType,
    super_prim: &PrimitiveType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::default();
    if super_prim.r#type == PrimitiveType::TABLE {
      result.is_subtype = true;
    }
    result
  }
}
