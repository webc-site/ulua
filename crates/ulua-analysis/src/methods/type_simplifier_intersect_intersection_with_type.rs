use ulua_common::dfint;

use crate::{
  enums::relation::Relation,
  functions::{
    add_intersection::add_intersection, begin_type::begin_intersection_type, get_type,
    is_type_variable::is_type_variable, relate_simplify::relate_type_id_type_id,
  },
  records::{
    intersection_type::IntersectionType, type_ids::TypeIds, type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_intersection_with_type(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let builtin_types = self.builtin_types.get();
    // C++ Simplify.cpp intersectIntersectionWithType: LUAU_ASSERT(leftIntersection)
    let left_intersection =
      get_type::get::<IntersectionType>(left).expect("left is IntersectionType");

    if left_intersection.parts.len() > dfint::LuauSimplificationComplexityLimit.get() as usize {
      return add_intersection(self.arena, self.builtin_types, &[left, right]);
    }

    let mut changed = false;
    let mut new_parts = TypeIds::new();
    // C++ 的 `for (TypeId part : leftIntersection)` 走防环且展平嵌套 intersection
    // 的 `TypeIterator`。
    for part in begin_intersection_type(left_intersection) {
      match relate_type_id_type_id(part, right) {
        Relation::Disjoint => return builtin_types.never_type,
        Relation::Coincident => new_parts.insert_type_id(part),
        Relation::Subset => new_parts.insert_type_id(part),
        Relation::Superset => {
          new_parts.insert_type_id(right);
          changed = true;
        }
        Relation::Intersects => {
          new_parts.insert_type_id(part);
          new_parts.insert_type_id(right);
          changed = true;
        }
      }
    }

    // It is sometimes the case that an intersection operation will result in
    // clipping a free type from the result.
    //
    // eg (number & 'a) & string --> never
    //
    // We want to only report the free types that are part of the result.
    for &part in &new_parts.order {
      if is_type_variable(part) {
        self.blocked_types.insert(part);
      }
    }

    if !changed {
      return left;
    }

    self.intersect_from_parts(new_parts)
  }
}
