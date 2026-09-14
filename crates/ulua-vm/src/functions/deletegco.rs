use core::ffi::c_void;

use crate::{
  functions::freeobj::freeobj,
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

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
