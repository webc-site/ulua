use ulua_vm::records::lua_state::LuaState;

/// 类型函数运行期禁止调用的统一错误文本（cpp `unsupportedFunction`）。
use crate::functions::throw_type_error::throw_type_error;
const UNSUPPORTED_MSG: &str = "this function is not supported in type functions";

/// 对应 C++ 原生 `static int unsupportedFunction(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2064`）。
pub(crate) fn unsupported_function(l: &mut LuaState) -> i32 {
  // throw_type_error 必然抛出不返回，故本函数体以 `!` 收尾。
  throw_type_error(l, format_args!("{UNSUPPORTED_MSG}"))
}
