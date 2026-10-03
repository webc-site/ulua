

use ulua_ast::records::location::Location;
use ulua_common::{fflag, functions::{c_str::cstr_cow, format::format}, records::variant::Variant5};
use ulua_vm::{functions::{lua_isstring::lua_isstring, lua_l_typename::lua_l_typename, lua_typename::lua_typename}};
use crate::records::{runtime_error::RuntimeError, type_function_error::TypeFunctionError};
use ulua_vm::records::lua_state::LuaState;
pub fn check_result_for_error(
  l: &mut LuaState,
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
      if l.get_top() == 0 {
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
      // `lua_isstring` 为安全只读入口：进入本分支前已确认栈深 != 0，-1 落在有效栈槽内。
      if lua_isstring(&*l, -1) != 0 {
        let err_str = l.to_str(-1).unwrap_or_default();
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
          // `lua_l_typename` 为安全只读入口：-1 落在有效栈槽（上一分支保证栈非空），
          // 其实现按该槽 TValue 的 tag 取串表项，不写栈。
          lua_l_typename(&*l, -1)
        } else {
          // `lua_typename` 为安全函数：实参 -1 是 LUA_TNONE 常量（与 C++ 上游同值传参），
          // 该取值命中 "no value" 静态表项，不触碰 l 所指状态。
          lua_typename(l.as_mut_ptr(), -1)
        };
        // Safety: 两个分支的返回值分别是 luaO_ 串表/getstr 结果或静态 "no value"、
        // TYPENAMES_BYTES 表项——均非空、NUL 结尾且静态存活，指针读取安全。
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
