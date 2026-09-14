//! The public, mlua-style raw C API surface (re-exported at `ulua_rt::ffi`).
//!
//! Mirrors `mlua::ffi`. Only the commonly used subset of the ulua C API is
//! surfaced here, for callers that need to drop down to the stack-machine level
//! (e.g. inside [`Lua::exec_raw`](crate::Lua::exec_raw) or
//! [`Lua::create_c_function`](crate::Lua::create_c_function)). The full ulua C
//! API lives in the `ulua-vm` crate.
//!
//! Everything here is `unsafe` to use and offers no safety guarantees beyond the
//! underlying VM — it is the same low-level interface the safe wrappers sit on
//! top of. ulua is a pure-Rust VM, so these are plain Rust `fn`s, not a C ABI
//! boundary (see [`lua_CFunction`], which is a Rust `unsafe fn`, not an
//! `extern "C-unwind" fn`).

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
