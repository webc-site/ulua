//! Source: `Analysis/src/Subtyping.cpp:1661-1700` —
//! `Subtyping::isCovariantWith(SubtypingEnvironment&, const UnionType*, const UnionType*, NotNull<Scope>)`.
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, variant::Variant},
  functions::{begin_type::begin_union_type, follow_type},
  records::{
    index::Index, scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, type_ids::TypeIds, union_type::UnionType,
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
    let mut result = SubtypingResult::ok();

    let mut super_union_options = TypeIds::new();
    super_union_options.reserve(super_union.options.len());
    // C++ `superUnionOptions.insert(begin(superUnion), end(superUnion))`
    // —— UnionTypeIterator 展平嵌套 union 并 follow,裸遍历 options 会漏掉
    // 嵌套成员。
    for ty in begin_union_type(super_union) {
      super_union_options.insert_type_id(ty);
    }

    for &ty in &super_union_options.order {
      LUAU_ASSERT!(ty == follow_type::follow(ty));
    }

    // C++ `for (TypeId ty : subUnion)` — UnionTypeIterator 防环并展平嵌套 union；
    // 裸遍历 options 会拿到未 follow 的绑定类型。
    for (sub_index, ty) in begin_union_type(sub_union).enumerate() {
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
          return SubtypingResult::too_complex();
        }
      }
    }

    result
  }
}
