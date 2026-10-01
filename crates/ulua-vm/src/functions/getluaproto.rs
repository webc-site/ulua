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
  // 既有约定（review.md §2）：VM c-API 边界签名折返——返回 `*mut Proto`，非 Lua 闭包时 null 为合法返回，边界体内保留裸指针
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
