use core::mem::size_of;

use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  macros::lua_m_freearray::luaM_freearray,
  records::{gc_object::GCObject, lua_page::lua_Page, luau_class::LuauClass, t_string::tstring},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn lua_r_freeclass(
  l: *mut lua_State,
  classobject: *mut LuauClass,
  page: *mut lua_Page,
) {
  unsafe {
    let numberof_all_members = (*classobject).numberofallmembers;
    let numberof_instance_members = (*classobject).numberofinstancemembers;
    let static_member_count = numberof_all_members - numberof_instance_members;

    luaM_freearray!(
      l,
      (*classobject).staticmembers,
      static_member_count,
      TValue,
      (*classobject).memcat
    );
    luaM_freearray!(
      l,
      (*classobject).offsettomember,
      numberof_all_members,
      *mut tstring,
      (*classobject).memcat
    );
    luaM_freegco_(
      l,
      classobject as *mut GCObject,
      size_of::<LuauClass>(),
      (*classobject).memcat,
      page,
    );
  }
}
