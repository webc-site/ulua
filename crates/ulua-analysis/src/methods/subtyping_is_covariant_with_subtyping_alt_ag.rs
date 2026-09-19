use crate::{
  records::{
    reduction::Reduction, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{component::Component, type_id::TypeId},
};
impl Subtyping {
  /// # Safety
  /// 调用方须保证 `scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_covariant_with_subtyping_environment_type_function_instance_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_function_instance: &TypeFunctionInstanceType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let (ty, mut errors) =
      unsafe { self.handle_type_function_reduction_result(sub_function_instance, scope) };

    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, ty, super_ty, scope,
      )
      .with_errors(&mut errors)
      .with_sub_component(Component::Reduction(Reduction { result_type: ty }))
      .to_owned()
  }
}
