//! Source: `VM/src/lstate.cpp:59-70` (hand-ported)

use core::ffi::c_void;

use crate::{
  functions::{
    lua_h_new::lua_h_new, lua_s_resize::lua_s_resize, lua_t_init::lua_t_init,
    stack_init::stack_init,
  },
  macros::{
    lua_errerrmsg::LUA_ERRERRMSG_STR, lua_memerrmsg::LUA_MEMERRMSG_STR,
    lua_minstrtabsize::LUA_MINSTRTABSIZE, lua_s_fix::luaS_fix, lua_s_newliteral::lua_s_newliteral,
    registry::registry, sethvalue::sethvalue,
  },
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 必须是 `lua_newstate` 刚创建、尚未 open 的主线程：其 `global`/分配器已就绪且 base_ci 有效；
/// 只能经 `lua_d_rawrunprotected` 作为受保护回调调用（内部 `stack_init`/`lua_s_resize` 可抛错 unwind）。
/// 对已 open 的状态重复调用会重建栈与 registry、泄漏旧表。cpp lstate.cpp:71。
/// open parts that may cause memory-allocation errors
pub(crate) unsafe extern "C-unwind" fn f_luaopen(l: *mut LuaState, _ud: *mut c_void) {
  // Safety: 此为 lua_newstate 打开期回调，仅使用 `l` 自身与已初始化的分配器字段建立 registry/基础库
  unsafe {
    let g = (*l).global;
    stack_init(l, l); // init stack
    (*l).gt = lua_h_new(l, 0, 2); // table of globals
    sethvalue!(
      l,
      registry!(l) as *const TValue as *mut TValue,
      lua_h_new(l, 0, 2)
    ); // registry
    lua_s_resize(l, LUA_MINSTRTABSIZE); // initial size of string table
    lua_t_init(l);
    luaS_fix!(lua_s_newliteral(l, LUA_MEMERRMSG_STR.as_bytes())); // pin to make sure we can always throw this error
    luaS_fix!(lua_s_newliteral(l, LUA_ERRERRMSG_STR.as_bytes())); // pin to make sure we can always throw this error
    (*g).gc_threshold = 4 * (*g).totalbytes;
  }
}
