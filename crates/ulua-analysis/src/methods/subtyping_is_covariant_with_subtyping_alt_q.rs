use crate::{
  functions::is_subclass_type::is_subclass_extern_type_extern_type,
  records::{
    extern_type::ExternType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_extern_type: &ExternType,
    super_extern_type: &ExternType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    SubtypingResult {
      is_subtype: is_subclass_extern_type_extern_type(sub_extern_type, super_extern_type),
      ..Default::default()
    }
  }
}
