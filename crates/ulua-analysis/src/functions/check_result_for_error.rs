use ulua_ast::records::location::Location;
use ulua_common::{
  fflag,
  functions::{c_str::cstr_cow, format::format},
  records::variant::Variant5,
};
use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_isstring::lua_isstring, lua_l_typename::lua_l_typename,
    lua_typename::lua_typename,
  },
  macros::lua_tostring::lua_tostring,
  records::lua_state,
};

use crate::{
  records::{runtime_error::RuntimeError, type_function_error::TypeFunctionError},
  type_aliases::lua_state::LuaState,
};
pub fn check_result_for_error(
  l: *mut LuaState,
  type_function_name: &str,
  lua_result: i32,
) -> Option<TypeFunctionError> {
  match lua_result {
    0 => None, // LuaOk
    1 | 3 => Some(
      TypeFunctionError::type_function_error_location_type_function_error_data(
        Location::new(Default::default(), Default::default()),
        Variant5::V2(RuntimeError::new(format(format_args!(
          "'{}' type function errored: unexpected yield or break",
          type_function_name
        )))),
      ),
    ), // LuaYield, LuaBreak
    _ => {
      // Safety: l 是类型函数调用边界传入的存活 lua_State（单线程执行、非空且对齐），
      // 读取 top/base 字段有效，与 C++ checkLastError(L) 的 L 生命周期一致。
      if unsafe { lua_gettop(l as *mut lua_state::LuaState) } == 0 {
        Some(
          TypeFunctionError::type_function_error_location_type_function_error_data(
            Location::new(Default::default(), Default::default()),
            Variant5::V2(RuntimeError::new(format(format_args!(
              "'{}' type function errored unexpectedly",
              type_function_name
            )))),
          ),
        )
      } else
      // Safety: l 为存活 lua_State；进入本分支前已确认栈深 != 0，-1 索引落在有效栈槽
      // 内，lua_isstring 只读取该槽位。
      if unsafe { lua_isstring(l as *mut lua_state::LuaState, -1) } != 0 {
        // Safety: 存活状态且栈非空；上一分支已确认 -1 槽是字符串，lua_tolstring 走
        // ttisstring 快路径直接返回串内容指针，不会进入转换/GC 重排路径。
        let err_str = unsafe { lua_tostring!(l as *mut lua_state::LuaState, -1) };
        // Safety: Lua 字符串在内存中恒以 NUL 终止且该分支下指针非空；取出到使用之间
        // 单线程无 GC、无栈槽移动，串内容保持存活。
        let err_str = unsafe { cstr_cow(err_str) };
        Some(
          TypeFunctionError::type_function_error_location_type_function_error_data(
            Location::new(Default::default(), Default::default()),
            Variant5::V2(RuntimeError::new(format(format_args!(
              "'{}' type function errored at runtime: {}",
              type_function_name, err_str
            )))),
          ),
        )
      } else {
        let err_type = if fflag::LuauUdtfFixTypeNameTypo.get() {
          // Safety: l 存活且栈非空（上一分支保证），-1 落在有效栈槽；luaL_typename
          // 只读该槽 TValue 并以判空/nilobject 兜底，永不误读空槽。
          unsafe { lua_l_typename(l as *mut lua_state::LuaState, -1) }
        } else {
          // `lua_typename` 为安全函数：实参 -1 是 LUA_TNONE 常量（与 C++ 上游同值传参），
          // 该取值命中 "no value" 静态表项，不触碰 l 所指状态。
          lua_typename(l as *mut lua_state::LuaState, -1)
        };
        // Safety: 两个分支的返回值分别是 luaO_ 串表/getstr 结果或静态 "no value"、
        // TYPENAMES_C 表项——均非空、NUL 结尾且静态存活，可安全构造 CStr。
        let err_type = unsafe { cstr_cow(err_type) };
        Some(
          TypeFunctionError::type_function_error_location_type_function_error_data(
            Location::new(Default::default(), Default::default()),
            Variant5::V2(RuntimeError::new(format(format_args!(
              "'{}' type function errored at runtime: raised an error of type {}",
              type_function_name, err_type
            )))),
          ),
        )
      }
    }
  }
}
