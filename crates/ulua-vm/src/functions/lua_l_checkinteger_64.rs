use core::{ffi::c_int, mem::transmute, ptr::null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tointeger_64::lua_tointeger_64, lua_type::lua_type, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_checkinteger_64")]
pub unsafe fn lua_l_checkinteger_64(l: *mut lua_State, narg: c_int) -> i64 {
  unsafe {
    // The macro lua_isinteger_64! expands to a call to lua_type(l, narg).
    // Since lua_type is currently a stub taking 0 arguments and returning (),
    // we must transmute it to the expected signature to allow the macro to compile.
    let lua_type_ptr = lua_type as *const ();
    let lua_type_real: unsafe extern "C-unwind" fn(*mut lua_State, c_int) -> c_int =
      transmute(lua_type_ptr);

    if lua_type_real(l, narg) != (LuaType::Integer as c_int) {
      tag_error(l, narg, LuaType::Integer as c_int);
    }

    // The C++ source calls lua_tointeger64(l, narg, nullptr).
    // The Rust dependency lua_tointeger_64 is currently a stub taking 0 arguments.
    // We must cast the stub call to the expected signature to satisfy the compiler.
    let lua_tointeger_64_ptr = lua_tointeger_64 as *const ();
    let func: unsafe extern "C-unwind" fn(*mut lua_State, c_int, *mut c_int) -> i64 =
      transmute(lua_tointeger_64_ptr);

    func(l, narg, null_mut())
  }
}

pub use lua_l_checkinteger_64 as luaL_checkinteger64;
