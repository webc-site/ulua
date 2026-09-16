//! Node: `cxx:Function:Luau.VM:VM/src/ldo.cpp:718:callerrfunc`
//! Source: `VM/src/ldo.cpp` (ldo.cpp:718-727, hand-ported)

use core::ffi::c_void;

use crate::{
  functions::lua_d_callny::lua_d_callny,
  macros::{incr_top::incr_top, setobj_2_s::setobj_2_s},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

pub(crate) unsafe extern "C-unwind" fn callerrfunc(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let errfunc = ud as StkId;

    setobj_2_s!(l, (*l).top, (*l).top.offset(-1));
    setobj_2_s!(l, (*l).top.offset(-1), errfunc);
    incr_top!(l);

    lua_d_callny(l, (*l).top.offset(-2), 1);
  }
}
