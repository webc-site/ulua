use crate::{
  functions::get_singleton_type::get_singleton_type,
  records::{
    boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType, scope::Scope,
    singleton_type::SingletonType, string_singleton::StringSingleton, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_singleton_type_primitive_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_singleton: &SingletonType,
    super_prim: &PrimitiveType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    if get_singleton_type::<StringSingleton>(sub_singleton).is_some()
      && super_prim.r#type == PrimitiveType::STRING
    {
      return SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    }

    if get_singleton_type::<BooleanSingleton>(sub_singleton).is_some()
      && super_prim.r#type == PrimitiveType::BOOLEAN
    {
      return SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    }

    SubtypingResult {
      is_subtype: false,
      ..Default::default()
    }
  }
}
