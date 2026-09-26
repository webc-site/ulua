use ulua_vm::{
  macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

/// 注册一个全局 C 函数的同形两步样板：
/// `LUA_PUSHCFUNCTION(l, f, name); lua_setglobal(l, name);`（把具名 C 闭包压栈后
/// 以同一名字挂到全局表）。cpp 各用例与 `*_setup` 夹具里逐字重复，收敛前本 crate 内
/// 有 10 份同形副本（`direct_field_access.rs` 8、`conformance_coverage_setup.rs` 1、
/// `conformance_tables_setup.rs` 1），现全部切到本门面。
///
/// 注意：仅「push 名 == setglobal 名」且目标是全局表的配对走此门面；压栈后落到
/// 元方法（`lua_setfield(l, -2, b"__index\0")`）或直接 `lua_pcall` 调用的形态不属本样板。
///
/// 前置条件（由用例保证）：`l` 是存活的 `LuaState`、`f` 遵循 Lua C 函数调用约定、`name` 为
/// 在函数注册存续期内有效、以 NUL 结尾的静态字节串（`b"name\0"` 字面量，与上游同一前提）。
pub fn push_cfunction_global(l: *mut LuaState, f: LuaCFunction, name: &'static [u8]) {
  // Safety: 逐字透传三个前置条件给被调的 LUA_PUSHCFUNCTION 宏与 lua_setglobal 宏，
  // 二者对 `l`/`f`/`name` 的要求与本门面文档一致。
  unsafe {
    LUA_PUSHCFUNCTION(l, f, name.as_ptr().cast());
    lua_setglobal(l, name.as_ptr().cast());
  }
}
