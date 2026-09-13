use core::mem::zeroed;

use crate::{
  enums::relation::Relation,
  functions::{
    add_intersection::add_intersection, get_type_alt_j::get_type_id,
    is_type_variable::is_type_variable, relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    any_type::AnyType, error_type::ErrorType, free_type::FreeType,
    intersection_type::IntersectionType, negation_type::NegationType, never_type::NeverType,
    recursion_limiter::RecursionLimiter, type_simplifier::TypeSimplifier, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn intersect(&mut self, mut left: TypeId, mut right: TypeId) -> TypeId {
    let mut rl = RecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    rl.recursion_limiter_recursion_limiter(
      "TypeSimplifier::intersect",
      &mut self.recursion_depth,
      15,
    );

    left = self.simplify_type_id(left);
    right = self.simplify_type_id(right);

    if left == right {
      return left;
    }

    let bt = unsafe { &*self.builtin_types };
    if get_type_id::<AnyType>(left).is_some() && get_type_id::<ErrorType>(right).is_some() {
      return right;
    }
    if get_type_id::<AnyType>(right).is_some() && get_type_id::<ErrorType>(left).is_some() {
      return left;
    }
    if get_type_id::<UnknownType>(left).is_some() && get_type_id::<ErrorType>(right).is_none() {
      return right;
    }
    if get_type_id::<UnknownType>(right).is_some() && get_type_id::<ErrorType>(left).is_none() {
      return left;
    }
    if get_type_id::<AnyType>(left).is_some() && get_type_id::<UnionType>(right).is_some() {
      return self.union_(bt.error_type, right);
    }
    if get_type_id::<UnionType>(left).is_some() && get_type_id::<AnyType>(right).is_some() {
      return self.union_(bt.error_type, left);
    }
    if get_type_id::<AnyType>(left).is_some() {
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return arena.add_type(UnionType {
        options: alloc::vec![right, bt.error_type],
      });
    }
    if get_type_id::<AnyType>(right).is_some() {
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return arena.add_type(UnionType {
        options: alloc::vec![left, bt.error_type],
      });
    }
    if get_type_id::<UnknownType>(left).is_some() {
      return right;
    }
    if get_type_id::<UnknownType>(right).is_some() {
      return left;
    }
    if get_type_id::<NeverType>(left).is_some() {
      return left;
    }
    if get_type_id::<NeverType>(right).is_some() {
      return right;
    }

    if let Some(lf) = get_type_id::<FreeType>(left) {
      let relation = relate_type_id_type_id(lf.upper_bound, right);
      if relation == Relation::Subset || relation == Relation::Coincident {
        return left;
      }
    } else if let Some(rf) = get_type_id::<FreeType>(right) {
      let relation = relate_type_id_type_id(left, rf.upper_bound);
      if relation == Relation::Superset || relation == Relation::Coincident {
        return right;
      }
    }

    if is_type_variable(left) {
      self.blocked_types.insert(left);
      return add_intersection(
        self.arena as *mut _,
        self.builtin_types as *mut _,
        &[left, right],
      );
    }
    if is_type_variable(right) {
      self.blocked_types.insert(right);
      return add_intersection(
        self.arena as *mut _,
        self.builtin_types as *mut _,
        &[left, right],
      );
    }

    if get_type_id::<UnionType>(left).is_some() {
      return if get_type_id::<UnionType>(right).is_some() {
        self.intersect_unions(left, right)
      } else {
        self.intersect_union_with_type(left, right)
      };
    } else if get_type_id::<UnionType>(right).is_some() {
      return self.intersect_union_with_type(right, left);
    }

    if get_type_id::<IntersectionType>(left).is_some() {
      return self.intersect_intersection_with_type(left, right);
    } else if get_type_id::<IntersectionType>(right).is_some() {
      return self.intersect_intersection_with_type(right, left);
    }

    if get_type_id::<NegationType>(left).is_some() {
      return if get_type_id::<NegationType>(right).is_some() {
        self.intersect_negations(left, right)
      } else {
        self.intersect_type_with_negation(left, right)
      };
    } else if get_type_id::<NegationType>(right).is_some() {
      return self.intersect_type_with_negation(right, left);
    }

    if let Some(res) = self.basic_intersect(left, right) {
      res
    } else {
      let arena = unsafe { &mut *self.arena.cast_mut() };
      arena.add_type(IntersectionType {
        parts: alloc::vec![left, right],
      })
    }
  }
}
