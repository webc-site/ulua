use core::ptr::{addr_of, null_mut};

use crate::{
  macros::{cast_to::cast_to, ci_func::ci_func, is_lua::isLua},
  records::{call_info::CallInfo, closure::LClosure},
  type_aliases::proto::Proto,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn get_lua_proto(ci: *mut CallInfo) -> *mut Proto {
  unsafe {
    if isLua!(ci) {
      let cl = ci_func!(ci);
      let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
      cast_to!(*mut Proto, (*lcl).p)
    } else {
      null_mut()
    }
  }
}
