//! Faithful port of
//! `std::optional<TypeFunctionError> TypeFunctionRuntime::registerFunction(AstStatTypeFunction* function)`
//! (Analysis/src/TypeFunctionRuntime.cpp:144-228)。
//! 核心逻辑与 `_deprecated` 版共享，见
//! [`super::type_function_runtime_register_function_impl`]。

use ulua_ast::records::{ast_stat_type_function::AstStatTypeFunction, location::Location};

use super::type_function_runtime_register_function_impl::RegisterErr;
use crate::{
  functions::check_result_for_error::check_result_for_error,
  records::{
    failed_to_compile::FailedToCompile, type_function_error::TypeFunctionError,
    type_function_missing::TypeFunctionMissing, type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::{lua_state::LuaState, type_function_error_data::TypeFunctionErrorData},
};

/// 新版错误表示：结构化 `TypeFunctionError`（V3 编译失败 / V4 缺失）。
impl RegisterErr for TypeFunctionError {
  fn compile_failed(name: String, what: String) -> Self {
    TypeFunctionError::type_function_error_location_type_function_error_data(
      Location::default(),
      TypeFunctionErrorData::V3(FailedToCompile::new(name, what)),
    )
  }
  fn missing(name: String) -> Self {
    TypeFunctionError::type_function_error_location_type_function_error_data(
      Location::default(),
      TypeFunctionErrorData::V4(TypeFunctionMissing::new(name)),
    )
  }
  fn check_result(l: *mut LuaState, name: &str, lua_result: i32) -> Option<Self> {
    check_result_for_error(l, name, lua_result)
  }
}

impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn register_function(
    &mut self,
    function: *mut AstStatTypeFunction,
  ) -> Option<TypeFunctionError> {
    unsafe { self.register_function_impl::<TypeFunctionError>(function) }
  }
}
