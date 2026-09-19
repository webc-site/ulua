use core::ffi::c_void;

use crate::{
  functions::freeobj::freeobj,
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn deletegco(
  context: *mut c_void,
  page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  unsafe {
    let l = context as *mut lua_State;
    freeobj(l, gco, page);
    true
  }
}
