use core::ptr::null_mut;

use crate::{
  macros::{cast_to::cast_to, ci_func::ci_func, is_lua::isLua},
  records::{call_info::CallInfo, closure::LClosure},
  type_aliases::proto::Proto,
};

pub(crate) unsafe fn get_lua_proto(ci: *mut CallInfo) -> *mut Proto {
  unsafe {
    if isLua!(ci) {
      let cl = ci_func!(ci);
      let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
      cast_to!(*mut Proto, (*lcl).p)
    } else {
      null_mut()
    }
  }
}
