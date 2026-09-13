use ulua_common::DFInt;

use crate::{
  enums::relation::Relation,
  functions::{
    add_intersection::add_intersection, get_type_alt_j::get_type_id,
    is_type_variable::is_type_variable, relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    intersection_type::IntersectionType, type_ids::TypeIds, type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_intersection_with_type(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let builtin_types = unsafe { &*self.builtin_types };
    // C++ Simplify.cpp intersectIntersectionWithType: LUAU_ASSERT(leftIntersection)
    let left_intersection =
      get_type_id::<IntersectionType>(left).expect("left is IntersectionType");

    if left_intersection.parts.len() > DFInt::LuauSimplificationComplexityLimit.get() as usize {
      return add_intersection(
        self.arena.cast_mut(),
        self.builtin_types as *mut _,
        &[left, right],
      );
    }

    let mut changed = false;
    let mut new_parts = TypeIds::new();
    for &part in &left_intersection.parts {
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
