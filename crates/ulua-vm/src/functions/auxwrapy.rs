use core::ffi::c_int;

use crate::{
  functions::{
    auxresume::auxresume, auxwrapfinish::auxwrapfinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::{
    cast_int::cast_int, co_status_break::CO_STATUS_BREAK, lua_upvalueindex::lua_upvalueindex,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_auxwrapy"))]
pub(crate) unsafe extern "C-unwind" fn auxwrapy(l: *mut lua_State) -> c_int {
  unsafe {
    let co = lua_tothread(l, lua_upvalueindex(1));
    let narg = cast_int!((*l).top.offset_from((*l).base));
    let r = auxresume(l, co, narg);
    if r == CO_STATUS_BREAK {
      interrupt_thread(l, co)
    } else {
      auxwrapfinish(l, r)
    }
  }
}
