use core::mem::size_of;

use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  records::{gc_object::GcObject, lua_page::lua_Page, up_val::UpVal},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn lua_f_freeupval(l: *mut lua_State, uv: *mut UpVal, page: *mut lua_Page) {
  unsafe {
    luaM_freegco_(
      l,
      uv as *mut GcObject,
      size_of::<UpVal>(),
      (*uv).hdr.memcat,
      page,
    );
  }
}
