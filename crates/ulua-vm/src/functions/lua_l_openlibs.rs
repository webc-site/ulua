//! Source: `VM/src/linit.cpp:42-60` (hand-ported)

use core::ptr::null;

use ulua_common::fflag;

use crate::{
  functions::{
    lua_call::lua_call, lua_pushlstring::lua_pushlstring, luaopen_base::luaopen_base,
    luaopen_bit_32::luaopen_bit32, luaopen_buffer::luaopen_buffer, luaopen_class::luaopen_class,
    luaopen_coroutine::luaopen_coroutine, luaopen_debug::luaopen_debug,
    luaopen_integer::luaopen_integer, luaopen_math::luaopen_math, luaopen_os::luaopen_os,
    luaopen_string::luaopen_string, luaopen_table::luaopen_table, luaopen_utf_8::luaopen_utf_8,
    luaopen_vector::luaopen_vector,
  },
  macros::lua_pushcfunction::LUA_PUSHCFUNCTION,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// 标准库注册表（cpp linit.cpp lualibs 全表，顺序即注册顺序）；末位 integer 条目
/// 受 `LuauIntegerLibrary` fflag 门控——关闭时只注册前 [`LUALIBS_BASE_LEN`] 项
/// （等价原 `lualibs_nointeger` 表），开启时全部注册。
const LUALIBS: [LuaLReg; 12] = [
  LuaLReg::new(b"", luaopen_base),
  LuaLReg::new(b"coroutine", luaopen_coroutine),
  LuaLReg::new(b"table", luaopen_table),
  LuaLReg::new(b"os", luaopen_os),
  LuaLReg::new(b"string", luaopen_string),
  LuaLReg::new(b"math", luaopen_math),
  LuaLReg::new(b"debug", luaopen_debug),
  LuaLReg::new(b"utf8", luaopen_utf_8),
  LuaLReg::new(b"bit32", luaopen_bit32),
  LuaLReg::new(b"buffer", luaopen_buffer),
  LuaLReg::new(b"vector", luaopen_vector),
  LuaLReg::new(b"integer", luaopen_integer),
];
/// integer 门控关闭时的注册面（不含末位 integer 条目）。
const LUALIBS_BASE_LEN: usize = 11;

/// # Safety
/// `l` 须为存活、已 `lua_newstate` 完成的全局状态机（含 registry/主线程）且处于受保护帧：对每个库
/// `LUA_PUSHCFUNCTION`+`lua_pushlstring(name)`+`lua_call(1,0)`，故 `(*l).top` 须逐轮留 ≥2 槽；各 luaopen_* 会再入 Lua、
/// 分配、注册全局表并可抛错/GC。空名 `b""` 表示注册到全局自身。仅初始化期单次调用，不应在受限/只读 env 上重复执行。
/// cpp VM/src/linit.cpp:42
pub unsafe fn lua_l_openlibs(l: *mut LuaState) {
  unsafe {
    // 编译期常量表 + 运行时切片定界取代原双运行时数组：条目均为实函数，
    // 无需 cpp linit.cpp:52 `lib->func` 的 None 哨兵收尾
    let len = if fflag::LuauIntegerLibrary.get() {
      LUALIBS.len()
    } else {
      LUALIBS_BASE_LEN
    };

    for lib in &LUALIBS[..len] {
      LUA_PUSHCFUNCTION(l, lib.func, null());
      lua_pushlstring(l, lib.name.as_ptr().cast(), lib.name.len());
      lua_call(l, 1, 0);
    }

    if fflag::DebugLuauUserDefinedClassesRuntime.get() {
      LUA_PUSHCFUNCTION(l, Some(luaopen_class), null());
      lua_pushlstring(l, b"class".as_ptr().cast(), 5);
      lua_call(l, 1, 0);
    }
  }
}
