//! Thin, centralized imports of the ulua `lua_*` API we build on (internal
//! plumbing).
//!
//! Every raw `lua_*` function/type/constant used by `ulua-rt` is re-exported
//! from here, so the rest of the crate has a single place to look and we keep
//! the (long) import paths in one module. Nothing in here is part of the public
//! API — the small raw surface we do expose lives in [`crate::api`].

pub(crate) use core::ffi::{c_char, c_int, c_void};

// ---- garbage collection --------------------------------------------------
pub(crate) use ulua_vm::enums::lua_gc_op::LuaGcOp;
// ---- interrupts / sandbox / memory categories (Luau) ---------------------
pub(crate) use ulua_vm::functions::lua_callbacks::lua_callbacks;
// ---- refs / call / load --------------------------------------------------
pub(crate) use ulua_vm::functions::lua_checkstack::lua_checkstack;
// ---- state ---------------------------------------------------------------
pub(crate) use ulua_vm::functions::lua_close::lua_close;
// ---- threads / coroutines ------------------------------------------------
pub(crate) use ulua_vm::functions::lua_costatus::lua_costatus;
// ---- tables --------------------------------------------------------------
pub(crate) use ulua_vm::functions::lua_createtable::lua_createtable;
// ---- raw table access + metatables + stack juggling ----------------------
pub(crate) use ulua_vm::functions::lua_equal::lua_equal;
// ---- function environment + debug info -----------------------------------
pub(crate) use ulua_vm::functions::lua_getfenv::lua_getfenv;
// ---- named registry + field access ---------------------------------------
pub(crate) use ulua_vm::functions::lua_getfield::lua_getfield;
// ---- stack / values ------------------------------------------------------
pub(crate) use ulua_vm::functions::lua_gettop::lua_gettop;
// ---- metatable-aware tostring --------------------------------------------
pub(crate) use ulua_vm::functions::lua_l_tolstring::lua_l_tolstring;
// ---- traceback -----------------------------------------------------------
pub(crate) use ulua_vm::functions::lua_l_traceback::lua_l_traceback;
// ---- buffers / vectors (Luau) --------------------------------------------
pub(crate) use ulua_vm::functions::lua_newbuffer::lua_newbuffer;
// ---- closures / userdata -------------------------------------------------
pub(crate) use ulua_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
// ---- async bridge (Future <-> coroutine) ---------------------------------
// Only the async feature touches these; gating them keeps the default build
// free of unused-import churn (and byte-identical).
#[cfg(feature = "async")]
pub(crate) use ulua_vm::functions::lua_pushinteger::lua_pushinteger;
// ---- light userdata ------------------------------------------------------
pub(crate) use ulua_vm::functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged;
// ---- named registry + integer-keyed raw table access ---------------------
pub(crate) use ulua_vm::functions::lua_rawgeti::lua_rawgeti;
#[cfg(feature = "async")]
pub(crate) use ulua_vm::functions::lua_tointegerx::lua_tointegerx;
// ---- macro-defined helpers (exposed as plain fns) ------------------------
pub(crate) use ulua_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
// ---- types ---------------------------------------------------------------
pub(crate) use ulua_vm::type_aliases::lua_c_function::lua_CFunction;
pub(crate) use ulua_vm::{
  functions::{
    lua_error::lua_error, lua_gc::lua_gc, lua_getinfo::lua_getinfo,
    lua_getmetatable::lua_getmetatable, lua_getreadonly::lua_getreadonly,
    lua_gettable::lua_gettable, lua_getupvalue::lua_getupvalue, lua_insert::lua_insert,
    lua_isyieldable::lua_isyieldable, lua_l_newstate::lua_l_newstate,
    lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread, lua_newthread::lua_newthread, lua_next::lua_next,
    lua_objlen::lua_objlen, lua_pcall::lua_pcall, lua_pushboolean::lua_pushboolean,
    lua_pushcclosurek::lua_pushcclosurek, lua_pushlstring::lua_pushlstring,
    lua_pushnil::lua_pushnil, lua_pushnumber::lua_pushnumber, lua_pushthread::lua_pushthread,
    lua_pushvalue::lua_pushvalue, lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_rawcheckstack::lua_rawcheckstack, lua_rawget::lua_rawget, lua_rawset::lua_rawset,
    lua_ref::lua_ref, lua_replace::lua_replace, lua_resetthread::lua_resetthread,
    lua_resume::lua_resume, lua_resumeerror::lua_resumeerror, lua_setfenv::lua_setfenv,
    lua_setfield::lua_setfield, lua_setmemcat::lua_setmemcat, lua_setmetatable::lua_setmetatable,
    lua_setreadonly::lua_setreadonly, lua_setsafeenv::lua_setsafeenv, lua_settable::lua_settable,
    lua_settop::lua_settop, lua_status::lua_status, lua_toboolean::lua_toboolean,
    lua_tobuffer::lua_tobuffer, lua_tolightuserdata::lua_tolightuserdata,
    lua_tolstring::lua_tolstring, lua_tonumberx::lua_tonumberx, lua_topointer::lua_topointer,
    lua_tothread::lua_tothread, lua_touserdata::lua_touserdata, lua_tovector::lua_tovector,
    lua_type::lua_type, lua_unref::lua_unref, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{
    lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX, lua_upvalueindex::lua_upvalueindex,
  },
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
};

// ---- 状态/类型标签（唯一真相在 ulua-vm 的枚举里，边界处 `as c_int`）------
pub(crate) use ulua_vm::enums::{
  lua_co_status::LuaCoStatus, lua_status::LuaStatus, lua_type::LuaType,
};
