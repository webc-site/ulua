use core::ffi::c_void;

use crate::{
  functions::validateobj::validateobj,
  records::{
    gc_object::GCObject, global_state::global_State, lua_page::lua_Page, lua_state::lua_State,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validategco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  unsafe {
    let l = context as *mut lua_State;
    let g: *mut global_State = (*l).global;

    validateobj(g, gco);

    false
  }
}
