use core::ffi::c_char;

use crate::{
  functions::lua_pushcclosurek::lua_pushcclosurek, records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

/// cpp `lua_pushcfunction`：转发到 `lua_pushcclosurek`（`nup=0`、无 continuation），
/// 与 C 宏语义一致。
///
/// # Safety
///
/// 经该函数指针调用时：`l` 必须指向存活的 `LuaState`；`f` 必须是 `Some` 且遵循 Lua
/// C 函数调用约定的指针；`debugname` 须为空指针或指向在闭包存活期内有效的 NUL 结尾
/// C 字符串（会被原样存入 `cc.debugname` 供后续读取）。
pub const LUA_PUSHCFUNCTION: unsafe fn(*mut LuaState, LuaCFunction, *const c_char) =
  // Safety: 闭包体按上述契约调用——`l` 存活、`f` 合约定、`debugname` 空或持久 NUL 串；nup=0 不耗栈槽
  |l, f, debugname| unsafe {
      lua_pushcclosurek(l, f, debugname, 0, None);
    };
