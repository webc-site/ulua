use core::{ffi::c_char, ptr::null};

use crate::{
  macros::getstr::getstr,
  records::{closure::Closure, proto::Proto},
};

/// # Safety
///
/// `l` 必须指向存活 `lua_State` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn getfuncname(cl: *mut Closure) -> *const c_char {
  // Safety: 契约保证 `ci` 为存活调用帧，向上回溯读取的 func 槽/原型常量索引均落在其所属结构界内
  unsafe {
    if cl.is_null() {
      return null();
    }

    if (*cl).is_c != 0 {
      let c_debugname = (*cl).inner.c.debugname;
      if !c_debugname.is_null() {
        c_debugname
      } else {
        null()
      }
    } else {
      let p: *mut Proto = (*cl).inner.l.p;

      if !p.is_null() {
        let p_debugname = (&(*p)).debugname;
        if !p_debugname.is_null() {
          getstr(p_debugname)
        } else {
          null()
        }
      } else {
        null()
      }
    }
  }
}
