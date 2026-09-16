//! Source: `Analysis/src/Subtyping.cpp:1661-1700` —
//! `Subtyping::isCovariantWith(SubtypingEnvironment&, const UnionType*, const UnionType*, NotNull<Scope>)`.
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, variant::Variant},
  functions::follow_type::follow_type_id,
  records::{
    index::Index, scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, type_ids::TypeIds, type_iterator::TypeIterator,
    union_type::UnionType,
  },
  type_aliases::component::Component,
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_union_type_union_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_union: &UnionType,
    super_union: &UnionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    // A | B | C <: D | E | F
    //
    // ... when all of A, B and C are subtypes of D | E | F. However, we can
    // optimize this (and avoid some correctness issues) by skipping any
    // options in the union subtype that are present in the union super type.
    // A trivial example is:
    //
    //  -- We can skip the `nil` part of the subtype.
    //  T? <: U? iff T <: U
    //
    // NOTE: The correct way to do this would be to unconditionally
    // semantically subtype unions.
    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };

    let mut super_union_options = TypeIds::new();
    super_union_options.reserve(super_union.options.len());
    for &ty in &super_union.options {
      super_union_options.insert_type_id(ty);
    }

    for &ty in &super_union_options.order {
      LUAU_ASSERT!(ty == follow_type_id(ty));
    }

    let mut sub_index = 0usize;
    let mut it =
      unsafe { TypeIterator::<UnionType>::type_iterator_type(sub_union as *const UnionType) };
    let end_it = TypeIterator::<UnionType>::type_iterator_default();
    while it.operator_ne(&end_it) {
      let ty = it.operator_deref();
      it.operator_inc();

      if super_union_options.count(ty) == 0 {
        let mut next = self
          .is_covariant_with_subtyping_environment_type_id_union_type_not_null_scope(
            env,
            ty,
            super_union,
            scope,
          );
        next.with_sub_component(Component::Index(Index {
          index: sub_index,
          variant: Variant::Union,
        }));
        result.and_also(next, SubtypingSuppressionPolicy::Any);
        if result.normalization_too_complex {
          return SubtypingResult {
            is_subtype: false,
            normalization_too_complex: true,
            ..Default::default()
          };
        }
      }

      sub_index += 1;
    }

    result
  }
}
