use core::mem::size_of;

use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  macros::lua_m_freearray::luaM_freearray,
  records::{gc_object::GCObject, lua_page::lua_Page, luau_object::LuauObject},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn lua_r_freeobject(
  l: *mut lua_State,
  classinstance: *mut LuauObject,
  page: *mut lua_Page,
) {
  unsafe {
    luaM_freearray!(
      l,
      (*classinstance).members,
      (*classinstance).numberofmembers,
      TValue,
      (*classinstance).memcat
    );

    luaM_freegco_(
      l,
      classinstance as *mut GCObject,
      size_of::<LuauObject>(),
      (*classinstance).memcat,
      page,
    );
  }
}
