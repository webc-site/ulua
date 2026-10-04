use alloc::string::String;

use ulua_common::functions::{c_str::cstr_cow, format::format};
use ulua_vm::{
  functions::{lua_isstring::lua_isstring, lua_typename::lua_typename},
  records::lua_state::LuaState,
};
pub fn check_result_for_error_deprecated(
  l: &mut LuaState,
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
      if l.get_top() == 0 {
        Some(format(format_args!(
          "'{}' type function errored unexpectedly",
          type_function_name
        )))
      } else if
      // `lua_isstring` 为安全只读入口：gettop != 0 保证 -1 为合法栈索引，
      // 其实现只做 lua_type 分类读取，不写栈。
      lua_isstring(l, -1) != 0 {
        let err_str = l.to_str(-1).unwrap_or_default();
        Some(format(format_args!(
          "'{}' type function errored at runtime: {}",
          type_function_name, err_str
        )))
      } else {
        // `lua_typename` 为安全函数：入参 -1（LUA_TNONE）是上游 cpp `lua_typename(L, -1)`
        // 的直译语义，命中本端口 "no value" 常量分支，返回存活 static c_char 数组；`l` 存活同上。
        let err_type = lua_typename(l.as_mut_ptr(), -1);
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
