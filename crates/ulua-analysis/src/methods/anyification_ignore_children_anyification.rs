use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{anyification::Anyification, extern_type::ExternType},
  type_aliases::type_id::TypeId,
};

impl Anyification {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let et = get_type_id::<ExternType>(ty);
    if !et.is_none() {
      return true;
    }

    unsafe { (*ty).persistent }
  }
}
