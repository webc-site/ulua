use core::ptr::{addr_of, null_mut};

use crate::{
  macros::{ci_func::ci_func, is_lua::isLua},
  records::{call_info::CallInfo, closure::LClosure, proto::Proto},
};

/// # Safety
/// `ci` 须为存活 `CallInfo`：仅当 `isLua!(ci)` 时经 `ci_func!` 取闭包并读 `inner.l` 内嵌 `LClosure.p`
/// （须为存活 `Proto`），否则返回 NULL——调用方须先判 isLua 或容忍 NULL 再解引用返回值。纯只读，不分配、不抛错。
/// cpp VM/src/ldebug.cpp:36
pub(crate) unsafe fn get_lua_proto(ci: *mut CallInfo) -> *mut Proto {
  unsafe {
    if isLua!(ci) {
      let cl = ci_func!(ci);
      let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
      (*lcl).p
    } else {
      null_mut()
    }
  }
}
