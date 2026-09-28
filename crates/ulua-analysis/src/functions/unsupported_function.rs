use ulua_vm::records::lua_state;

use crate::{functions::throw_type_error::throw_type_error, type_aliases::lua_state::LuaState};

/// 类型函数运行期禁止调用的统一错误文本（cpp `unsupportedFunction`）。
const UNSUPPORTED_MSG: &str = "this function is not supported in type functions";

/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int unsupportedFunction(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2064`）。
pub unsafe fn unsupported_function(l: *mut LuaState) -> i32 {
  // Safety: `l` 的有效性由本函数前置条件给出；消息为无占位符的静态串，
  // throw_type_error 必然抛出不返回，故本函数体以 `!` 收尾。
  unsafe {
    throw_type_error(
      l as *mut lua_state::LuaState,
      format_args!("{UNSUPPORTED_MSG}"),
    )
  }
}
