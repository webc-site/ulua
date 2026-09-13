use crate::records::{
  primitive_type::PrimitiveType, scope::Scope, subtyping::Subtyping,
  subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_primitive_type_primitive_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_prim: &PrimitiveType,
    super_prim: &PrimitiveType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    SubtypingResult {
      is_subtype: sub_prim.r#type == super_prim.r#type,
      ..Default::default()
    }
  }
}
