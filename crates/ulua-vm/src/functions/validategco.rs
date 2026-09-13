use core::ffi::c_void;

use crate::{
  functions::validateobj::validateobj,
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::lua_State},
  type_aliases::global_state::global_State,
};

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
