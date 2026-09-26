use crate::{
  functions::get_type_utils::get_optional_ty,
  records::{extern_type::ExternType, function_type::FunctionType, instantiation::Instantiation},
  type_aliases::type_id::TypeId,
};

impl Instantiation {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    // 对齐 cpp Instantiation.cpp:49-55：FunctionType 读取改走 log 的 pending 态
    // （txn_log_get_mutable）；ExternType 在 cpp 中本就是 plain get，保持不变。
    let log = self.base.base.log;
    let ft = unsafe { (*log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    if !ft.is_null() {
      return true;
    }

    let et = unsafe { get_optional_ty::<ExternType, TypeId>(Some(ty)) };
    !et.is_null()
  }
}
