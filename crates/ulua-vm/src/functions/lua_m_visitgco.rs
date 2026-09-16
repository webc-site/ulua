use core::ffi::c_void;

use crate::{
  functions::lua_m_visitpage::lua_m_visitpage,
  records::lua_page::lua_Page,
  type_aliases::{global_state::global_State, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_visitgco(
  l: *mut lua_State,
  context: *mut c_void,
  visitor: *mut c_void, // function pointer: bool (*)(void* context, lua_Page* page, GCObject* gco)
) {
  unsafe {
    let g: *mut global_State = (*l).global;

    let mut curr: *mut lua_Page = (*g).allgcopages;

    while !curr.is_null() {
      let next: *mut lua_Page = (*curr).listnext; // block visit might destroy the page

      lua_m_visitpage(curr, context, visitor);

      curr = next;
    }
  }
}
