//! Source: `VM/src/linit.cpp:42-60` (hand-ported)

use core::ptr::null;

use ulua_common::fflag;

use crate::{
  functions::{
    lua_call::lua_call, lua_pushlstring::lua_pushlstring, luaopen_base::luaopen_base_arm,
    luaopen_bit_32::luaopen_bit32_arm, luaopen_buffer::luaopen_buffer_arm,
    luaopen_class::luaopen_class_arm, luaopen_coroutine::luaopen_coroutine_arm,
    luaopen_debug::luaopen_debug_arm, luaopen_integer::luaopen_integer_arm,
    luaopen_math::luaopen_math_arm, luaopen_os::luaopen_os_arm, luaopen_string::luaopen_string_arm,
    luaopen_table::luaopen_table_arm, luaopen_utf_8::luaopen_utf_8_arm,
    luaopen_vector::luaopen_vector_arm,
  },
  macros::lua_pushcfunction::LUA_PUSHCFUNCTION,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// 标准库注册表（cpp linit.cpp lualibs 全表，顺序即注册顺序）；末位 integer 条目
/// 受 `LuauIntegerLibrary` fflag 门控——关闭时只注册前 [`LUALIBS_BASE_LEN`] 项
/// （等价原 `lualibs_nointeger` 表），开启时全部注册。
const LUALIBS: [LuaLReg; 12] = [
  LuaLReg::new(b"", luaopen_base_arm),
  LuaLReg::new(b"coroutine", luaopen_coroutine_arm),
  LuaLReg::new(b"table", luaopen_table_arm),
  LuaLReg::new(b"os", luaopen_os_arm),
  LuaLReg::new(b"string", luaopen_string_arm),
  LuaLReg::new(b"math", luaopen_math_arm),
  LuaLReg::new(b"debug", luaopen_debug_arm),
  LuaLReg::new(b"utf8", luaopen_utf_8_arm),
  LuaLReg::new(b"bit32", luaopen_bit32_arm),
  LuaLReg::new(b"buffer", luaopen_buffer_arm),
  LuaLReg::new(b"vector", luaopen_vector_arm),
  LuaLReg::new(b"integer", luaopen_integer_arm),
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
      LUA_PUSHCFUNCTION(l, Some(luaopen_class_arm), null());
      lua_pushlstring(l, b"class".as_ptr().cast(), 5);
      lua_call(l, 1, 0);
    }
  }
}
