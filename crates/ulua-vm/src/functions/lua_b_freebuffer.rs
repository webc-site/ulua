use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  macros::{obj_2_gco::obj2gco, sizebuffer::sizebuffer},
  records::{lua_page::lua_Page, luau_buffer::LuauBuffer as Buffer},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_b_freebuffer(l: *mut lua_State, b: *mut Buffer, page: *mut lua_Page) {
  unsafe {
    luaM_freegco_(
      l,
      obj2gco!(b),
      sizebuffer((*b).len as usize),
      (*b).memcat,
      page,
    );
  }
}
