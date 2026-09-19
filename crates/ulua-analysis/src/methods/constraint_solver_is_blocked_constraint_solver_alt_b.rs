use core::ffi::c_void;

use crate::{
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id},
  records::{
    blocked_type_pack::BlockedTypePack, constraint_solver::ConstraintSolver,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_blocked_type_pack_id(&self, tp: TypePackId) -> bool {
    let tp = unsafe { follow_type_pack_id(tp) };

    if !get_type_pack_id::<TypeFunctionInstanceTypePack>(tp).is_none() {
      return !self
        .uninhabited_type_functions
        .contains(&(tp as *const c_void));
    }

    !get_type_pack_id::<BlockedTypePack>(tp).is_none()
  }
}
