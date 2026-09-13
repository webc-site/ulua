use crate::{
  enums::{pack_field::PackField, type_field::TypeField},
  records::{
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{component::Component, type_pack_id::TypePackId},
};
impl Subtyping {
  pub fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_variadic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
    _sub_tp: TypePackId,
    sub: &VariadicTypePack,
    _super_tp: TypePackId,
    super_variadic: &VariadicTypePack,
  ) -> SubtypingResult {
    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub.ty,
        super_variadic.ty,
        scope,
      )
      .with_both_component(Component::TypeField(TypeField::Variadic))
      .with_both_component(Component::PackField(PackField::Tail))
      .to_owned()
  }
}
