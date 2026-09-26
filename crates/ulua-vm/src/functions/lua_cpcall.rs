use core::ffi::c_void;

use crate::{
  functions::{f_ccall::f_ccall, lua_d_pcall::lua_d_pcall},
  macros::{api_check::api_check, savestack::savestack},
  records::{c_call_s::CCallS, lua_state::LuaState},
  type_aliases::lua_c_function::LuaCFunction,
};

/// # Safety
/// `l` 须为存活 LuaState、`(*l).status == 0` 且处于可建立新保护帧的位置（`api_check` 仅 debug 断言）；
/// `func` 须为 `Some` 的 C 函数，`ud` 作为 light userdata 透传给 `f_ccall`（可为空指针，由被调 C 函数解释）；
/// `savestack!((*l).top)` 记录栈偏移，`lua_d_pcall` 期间栈可重分配。cpp/VM/src/lapi.cpp:1225 lua_cpcall。
pub unsafe fn lua_cpcall(l: *mut LuaState, func: LuaCFunction, ud: *mut c_void) -> i32 {
  unsafe {
    api_check!(l, (*l).status == 0);
    api_check!(l, func.is_some());

    let mut c = CCallS { func, ud };

    lua_d_pcall(
      l,
      Some(f_ccall),
      &mut c as *mut CCallS as *mut c_void,
      savestack!(l, (*l).top) as isize,
      0,
    )
  }
}
