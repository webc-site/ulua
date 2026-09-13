use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, variant::Variant},
  records::{
    index::Index, intersection_type::IntersectionType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    type_iterator::TypeIterator,
  },
  type_aliases::{component::Component, type_id::TypeId},
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_type_id_intersection_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_intersection: &IntersectionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };
    let mut i = 0usize;

    let mut it = unsafe {
      TypeIterator::<IntersectionType>::type_iterator_type(super_intersection as *const _)
    };
    let end_it = TypeIterator::<IntersectionType>::type_iterator_default();
    while it.operator_ne(&end_it) {
      let ty = it.operator_deref();
      it.operator_inc();

      let mut candidate = self
        .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub_ty, ty, scope,
        );
      candidate.with_super_component(Component::Index(Index {
        index: i,
        variant: Variant::Intersection,
      }));
      result.and_also(candidate, SubtypingSuppressionPolicy::Any);
      i += 1;

      if result.normalization_too_complex {
        return SubtypingResult {
          is_subtype: false,
          normalization_too_complex: true,
          ..Default::default()
        };
      }
    }

    result
  }
}
