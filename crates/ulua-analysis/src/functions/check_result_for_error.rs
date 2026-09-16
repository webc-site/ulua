use core::ffi::{CStr, c_int};

use ulua_ast::records::location::Location;
use ulua_common::{FFlag, functions::format::format, records::variant::Variant5};
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
  lua_result: c_int,
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
      } else if unsafe { lua_isstring(l as *mut lua_state::LuaState, -1) } != 0 {
        let err_str = unsafe { lua_tostring!(l as *mut lua_state::LuaState, -1) };
        let err_str = unsafe { CStr::from_ptr(err_str).to_string_lossy() };
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
        let err_type = if FFlag::LuauUdtfFixTypeNameTypo.get() {
          unsafe { lua_l_typename(l as *mut lua_state::LuaState, -1) }
        } else {
          unsafe { lua_typename(l as *mut lua_state::LuaState, -1) }
        };
        let err_type = unsafe { CStr::from_ptr(err_type).to_string_lossy() };
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
