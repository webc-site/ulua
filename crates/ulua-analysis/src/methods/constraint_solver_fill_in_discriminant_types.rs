use crate::{
  functions::{as_mutable_type::as_mutable_type_id, follow_type::follow_type_id},
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn fill_in_discriminant_types(
    &mut self,
    constraint: *const Constraint,
    discriminant_types: &[Option<TypeId>],
  ) {
    for ty_opt in discriminant_types.iter() {
      let ty = match ty_opt {
        Some(ty) => *ty,
        None => continue,
      };

      let follow_ty = follow_type_id(ty);

      if self.is_blocked_type_id(follow_ty) {
        let mutable_ty = { as_mutable_type_id(follow_ty) };
        unsafe {
          (*mutable_ty).ty = TypeVariant::Bound((*self.builtin_types).no_refine_type);
        }
      }

      let constraint_loc = unsafe { (*constraint).location };
      self.unblock_type_id_location(ty, constraint_loc);
    }
  }
}
