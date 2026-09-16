use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, intersection_type::IntersectionType,
    refinement_partition::RefinementPartition, scope::Scope,
  },
  type_aliases::{
    constraint_v::ConstraintV, refinement_context::RefinementContext, scope_ptr_type::ScopePtr,
  },
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn union_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    lhs: &RefinementContext,
    rhs: &RefinementContext,
    dest: *mut RefinementContext,
    _constraints: *mut Vec<ConstraintV>,
  ) {
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

    for (def, partition) in lhs.iter() {
      let rhs_partition = match rhs.get(def) {
        Some(p) => p,
        None => continue,
      };

      LUAU_ASSERT!(!partition.discriminant_types.is_empty());
      LUAU_ASSERT!(!rhs_partition.discriminant_types.is_empty());

      // C++ `intersect(types)`: 1 -> the sole type, 2 -> makeIntersect, more -> an IntersectionType.
      let left_discriminant_ty = {
        let types = &partition.discriminant_types;
        if types.len() == 1 {
          types[0]
        } else if types.len() == 2 {
          self.make_intersect(scope, location, types[0], types[1])
        } else {
          unsafe {
            (*self.arena).add_type(IntersectionType {
              parts: types.clone(),
            })
          }
        }
      };

      let right_discriminant_ty = {
        let types = &rhs_partition.discriminant_types;
        if types.len() == 1 {
          types[0]
        } else if types.len() == 2 {
          self.make_intersect(scope, location, types[0], types[1])
        } else {
          unsafe {
            (*self.arena).add_type(IntersectionType {
              parts: types.clone(),
            })
          }
        }
      };

      let union_ty = self.make_union_scope_ptr_location_type_id_type_id(
        scope_raw,
        location,
        left_discriminant_ty,
        right_discriminant_ty,
      );

      let should_append_nil =
        partition.should_append_nil_type || rhs_partition.should_append_nil_type;

      unsafe {
        (*dest).insert(*def, RefinementPartition::default());
        let dest_partition = (*dest).get_mut(def).unwrap();
        dest_partition.discriminant_types.push(union_ty);
        dest_partition.should_append_nil_type |= should_append_nil;
      }
    }
  }
}
