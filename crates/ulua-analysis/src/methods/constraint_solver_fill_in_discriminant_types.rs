use crate::{
  functions::{as_mutable_type::as_mutable_type_id, follow_type},
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};

impl ConstraintSolver {
  pub fn fill_in_discriminant_types(
    &mut self,
    constraint: &Constraint,
    discriminant_types: &[Option<TypeId>],
  ) {
    for ty_opt in discriminant_types.iter() {
      let ty = match ty_opt {
        Some(ty) => *ty,
        None => continue,
      };

      let follow_ty = follow_type::follow(ty);

      if self.is_blocked_type_id(follow_ty) {
        let mutable_ty = { as_mutable_type_id(follow_ty) };
        // SAFETY: as_mutable_type_id 返回 arena 内有效对象。
        unsafe {
          (*mutable_ty).ty = TypeVariant::Bound(self.builtin_types_ref().no_refine_type);
        }
      }

      self.unblock_type_id_location(ty, constraint.location);
    }
  }
}
