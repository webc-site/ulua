use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, variant::Variant},
  records::{
    index::Index, scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, type_iterator::TypeIterator, union_type::UnionType,
  },
  type_aliases::{component::Component, type_id::TypeId},
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_union_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_union: &UnionType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };
    let mut index = 0usize;
    let mut it =
      unsafe { TypeIterator::<UnionType>::type_iterator_type(sub_union as *const UnionType) };
    let end_it = TypeIterator::<UnionType>::type_iterator_default();
    while it.operator_ne(&end_it) {
      let ty = it.operator_deref();
      it.operator_inc();

      let mut next = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, ty, super_ty, scope,
      );
      if next.normalization_too_complex {
        return SubtypingResult {
          is_subtype: false,
          normalization_too_complex: true,
          ..Default::default()
        };
      }
      next.with_sub_component(Component::Index(Index {
        index,
        variant: Variant::Union,
      }));
      result.and_also(next, SubtypingSuppressionPolicy::Any);
      index += 1;
    }
    result
  }
}
