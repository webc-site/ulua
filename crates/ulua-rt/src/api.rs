//! The public, raw `lua_*` API surface (`ulua_rt::api`) — ulua's counterpart to
//! `mlua::ffi`. Only the commonly used subset of the ulua C API is surfaced
//! here, for callers that need to drop down to the stack-machine level
//! (e.g. inside [`Lua::exec_raw`](crate::Lua::exec_raw) or
//! [`Lua::create_c_function`](crate::Lua::create_c_function)). The full ulua C
//! API lives in the `ulua-vm` crate.
//!
//! Everything here is `unsafe` to use and offers no safety guarantees beyond the
//! underlying VM — it is the same low-level interface the safe wrappers sit on
//! top of. ulua is a pure-Rust VM: these are ordinary Rust `pub unsafe fn`s
//! re-exported from `ulua-vm`, not declarations behind a C ABI boundary. The one
//! C-ABI-shaped item is [`lua_CFunction`](crate::api::lua_CFunction), the VM's
//! native-callback pointer type (`unsafe extern "C-unwind" fn`), kept for
//! signature parity with the VM.

pub use ulua_vm::{
  functions::{
    lua_call::lua_call, lua_error::lua_error, lua_getfield::lua_getfield, lua_pcall::lua_pcall,
    lua_pushboolean::lua_pushboolean, lua_pushinteger::lua_pushinteger,
    lua_pushnumber::lua_pushnumber, lua_setfield::lua_setfield, lua_tointegerx::lua_tointegerx,
    lua_tonumberx::lua_tonumberx, lua_type::lua_type,
  },
  macros::{lua_getglobal::lua_getglobal, lua_setglobal::lua_setglobal},
  type_aliases::{lua_c_function::lua_CFunction, lua_state::lua_State},
};
