use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  macros::{obj_2_gco::obj2gco, sizebuffer::sizebuffer},
  records::lua_page::lua_Page,
  type_aliases::{buffer::Buffer, lua_state::lua_State},
};

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
