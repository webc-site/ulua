use core::ffi::c_int;

use crate::{
  functions::lua_yield::lua_yield, macros::cast_int::cast_int, type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_coyield")]
pub(crate) unsafe extern "C-unwind" fn coyield(l: *mut lua_State) -> c_int {
  unsafe {
    let nres = cast_int!((*l).top.offset_from((*l).base));
    lua_yield(l, nres)
  }
}
