//! Faithful port of
//! `std::optional<std::string> TypeFunctionRuntime::registerFunction_DEPRECATED(AstStatTypeFunction* function)`
//! (Analysis/src/TypeFunctionRuntime.cpp:58-142)。
//! 核心逻辑与新版共享，见
//! [`super::type_function_runtime_register_function_impl`]。
use alloc::string::String;

use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;
use ulua_common::functions::format::format;

use super::type_function_runtime_register_function_impl::RegisterErr;
use crate::{
  functions::check_result_for_error_deprecated::check_result_for_error_deprecated,
  records::type_function_runtime::TypeFunctionRuntime, type_aliases::lua_state::LuaState,
};

/// 旧版错误表示：格式化字符串（与 cpp 各 format 调用逐字对应）。
impl RegisterErr for String {
  fn compile_failed(name: String, what: String) -> Self {
    format(format_args!(
      "'{}' type function failed to compile with error message: {}",
      name, what
    ))
  }
  fn missing(name: String) -> Self {
    format(format_args!(
      "Could not find '{}' type function in the global scope",
      name
    ))
  }
  fn check_result(l: *mut LuaState, name: &str, lua_result: i32) -> Option<Self> {
    check_result_for_error_deprecated(l, name, lua_result)
  }
}

impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn register_function_deprecated(
    &mut self,
    function: *mut AstStatTypeFunction,
  ) -> Option<String> {
    unsafe { self.register_function_impl::<String>(function) }
  }
}
