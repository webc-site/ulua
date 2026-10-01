use core::ffi::c_char;

use crate::{
  functions::lua_l_getmetafield::{lua_l_getmetafield, lua_l_getmetafield_bytes},
  macros::abs_index::abs_index,
  records::lua_state::LuaState,
};

/// [`lua_l_callmeta`] 的字节切片核心（§10：Rust 内部调用方一律走此形，不构造 NUL 结尾缓冲）。
///
/// # Safety
/// `l` 须为存活 LuaState 并处于可调用/可 GC 的受保护帧；`obj` 为 `abs_index` 归一后的合法栈索引（`lua_pushvalue`/元方法入栈读该槽），
/// `event` 为元方法键字节切片；命中元方法后 `lua_call` 建立新帧、可抛错。
/// cpp/VM/src/laux.cpp:297 luaL_callmeta。
pub(crate) unsafe fn lua_l_callmeta_bytes(l: *mut LuaState, obj: i32, event: &[u8]) -> i32 {
  unsafe {
    let obj = abs_index(&*l, obj);
    if lua_l_getmetafield_bytes(l, obj, event) == 0 {
      return 0;
    }

    (*l).push_value(obj);
    (*l).call(1, 1);
    1
  }
}

/// # Safety
/// `l` 须为存活 LuaState 并处于可调用/可 GC 的受保护帧；`obj` 为 `abs_index` 归一后的合法栈索引（`lua_pushvalue`/元方法入栈读该槽），
/// `event` 须为 NUL 结尾 C 串或 null（C ABI 契约经 [`lua_l_getmetafield`] 保留 null 键分支），
/// 非空即与 [`lua_l_callmeta_bytes`] 同路径；命中元方法后 `lua_call` 建立新帧、可抛错。
/// cpp/VM/src/laux.cpp:297 luaL_callmeta。
pub unsafe fn lua_l_callmeta(l: *mut LuaState, obj: i32, event: *const c_char) -> i32 {
  unsafe {
    let obj = abs_index(&*l, obj);
    if lua_l_getmetafield(l, obj, event) == 0 {
      return 0;
    }

    (*l).push_value(obj);
    (*l).call(1, 1);
    1
  }
}
