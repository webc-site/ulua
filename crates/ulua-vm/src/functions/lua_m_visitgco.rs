use core::ffi::c_void;

use crate::{
  functions::lua_m_visitpage::lua_m_visitpage,
  records::{global_state::global_State, lua_page::lua_Page, lua_state::LuaState},
  type_aliases::gc_visitor::GcVisitor,
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global.allgcopages` 为完整页链表（每页 `listnext` 有效，访问前取 `next` 以防被回调销毁）；
/// `visitor` 须为可调用的回调，`context` 原样透传（可为空由 visitor 解释）。遍历全部 GC 页、逐对象回调，须在独占/调试上下文调用。
/// cpp/VM/src/lmem.cpp:882 luaM_visitgco。
pub unsafe fn lua_m_visitgco(l: *mut LuaState, context: *mut c_void, visitor: GcVisitor) {
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
