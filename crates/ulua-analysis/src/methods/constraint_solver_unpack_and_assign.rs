use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_solver::ConstraintSolver,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintSolver {
  pub fn unpack_and_assign(
    &mut self,
    dest_types: Vec<TypeId>,
    src_types: TypePackId,
    constraint: NonNull<Constraint>,
  ) -> NonNull<Constraint> {
    let c = self.push_constraint(
      NonNull::new(unsafe { (*constraint.as_ptr()).scope }).unwrap(),
      unsafe { (*constraint.as_ptr()).location },
      ConstraintV::Unpack(UnpackConstraint {
        result_pack: dest_types.clone(),
        source_pack: src_types,
      }),
    );

    for t in dest_types {
      // C++ `LUAU_ASSERT(bt)` 后 `bt->replaceOwner(...)`：dest 均为 blocked。
      get_mutable_type_id::<BlockedType>(t)
        .expect("unpack dest must be blocked")
        .replace_owner(c.as_ptr());
    }

    c
  }
}
