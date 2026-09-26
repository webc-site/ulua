use alloc::string::String;

use ulua_common::functions::{c_str::cstr_cow, format::format};
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_isstring::lua_isstring, lua_typename::lua_typename},
  macros::lua_tostring::lua_tostring,
  records::lua_state,
};

use crate::type_aliases::lua_state::LuaState;
pub fn check_result_for_error_deprecated(
  l: *mut LuaState,
  type_function_name: &str,
  lua_result: i32,
) -> Option<String> {
  match lua_result {
    0 => None, // LuaOk
    1 | 3 => Some(format(format_args!(
      "'{}' type function errored: unexpected yield or break",
      type_function_name
    ))), // LuaYield, LuaBreak
    _ => {
      // Safety: `l` 是类型函数 `luau_load`/`lua_resume` 同步调用链（cpp
      // TypeFunctionRuntime.cpp:138-142 同款时机）传入的存活 lua_State 指针，
      // 非空对齐且在本函数返回前独占存活；`as *mut lua_state::LuaState` 只是
      // 从 opaque `LuaState` 还原同一指针真型，不改地址。
      // lua_gettop 仅读取该状态的栈顶差。
      if unsafe { lua_gettop(l as *mut lua_state::LuaState) } == 0 {
        Some(format(format_args!(
          "'{}' type function errored unexpectedly",
          type_function_name
        )))
      } else if unsafe {
        // Safety: 同上，`l` 存活；gettop != 0 保证 -1 为合法栈索引，
        // lua_isstring 只做 lua_type 分类读取。
        lua_isstring(l as *mut lua_state::LuaState, -1) != 0
      } {
        // Safety: `l`/索引 -1 合法性同上；lua_isstring != 0 说明栈顶为字符串
        // 或数字，lua_tolstring 对二者均完成转换后返回 TString 缓冲（非空且以
        // '\0' 结尾），len 出参传 null 是其允许形态。
        let err_str = unsafe { lua_tostring!(l as *mut lua_state::LuaState, -1) };
        // Safety: err_str 非空且指向 NUL 结尾的 TString 缓冲（上一条证成），
        // to_string_lossy 只在本语句内读取。
        let err_str = unsafe { cstr_cow(err_str) };
        Some(format(format_args!(
          "'{}' type function errored at runtime: {}",
          type_function_name, err_str
        )))
      } else {
        // `lua_typename` 为安全函数：入参 -1（LUA_TNONE）是上游 cpp `lua_typename(L, -1)`
        // 的直译语义，命中本端口 "no value" 常量分支，返回存活 static c_char 数组；`l` 存活同上。
        let err_type = lua_typename(l as *mut lua_state::LuaState, -1);
        // Safety: err_type 为非空 NUL 结尾 static 字符串（上一条证成）。
        let err_type = unsafe { cstr_cow(err_type) };
        Some(format(format_args!(
          "'{}' type function errored at runtime: raised an error of type {}",
          type_function_name, err_type
        )))
      }
    }
  }
}
