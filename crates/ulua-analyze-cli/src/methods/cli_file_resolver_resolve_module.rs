use alloc::{boxed::Box, string::String};
/// `Luau::Require::ErrorHandler nullErrorHandler{};` — a no-op error handler
/// (`CLI/src/Analyze.cpp:207`).
use core::slice::from_raw_parts;

use ulua_analysis::records::{module_info::ModuleInfo, type_check_limits::TypeCheckLimits};
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_require::{
  enums::status_require_navigator::Status,
  records::{error_handler::ErrorHandler, navigator::Navigator},
};

use crate::{
  methods::{
    file_navigation_context_get_identifier::file_navigation_context_get_identifier,
    file_navigation_context_is_module_present::file_navigation_context_is_module_present,
  },
  records::{
    cli_file_resolver::CliFileResolver, file_navigation_context::FileNavigationContext,
    luau_config_interrupt_info::LuauConfigInterruptInfo,
  },
};
struct NullErrorHandler;

impl ErrorHandler for NullErrorHandler {
  fn report_error(&mut self, _message: String) {}
}

impl CliFileResolver {
  /// C++ `std::optional<ModuleInfo> resolveModule(const ModuleInfo* context, AstExpr* node, const TypeCheckLimits& limits)`
  /// (`CLI/src/Analyze.cpp:185-221`).
  ///
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn resolve_module(
    &mut self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    unsafe {
      // if (AstExprConstantString* expr = node->as<AstExprConstantString>())
      let const_str = ast_node_as::<AstExprConstantString>(expr as *mut AstNode);
      if const_str.is_null() {
        return None;
      }

      // std::string path{expr->value.data, expr->value.size};
      let bytes: &[u8] = {
        let slice = (*const_str).value.as_slice();
        from_raw_parts(slice.as_ptr() as *const u8, slice.len())
      };
      let path = String::from_utf8_lossy(bytes).into_owned();

      // FileNavigationContext navigationContext{context->name};
      let mut navigation_context = FileNavigationContext::new((*context).name.clone());

      // LuauConfigInterruptInfo info = {limits, path};
      // navigationContext.luauConfigInit / luauConfigInterrupt capture &info.
      navigation_context.interrupt_info = Some(Box::new(LuauConfigInterruptInfo {
        limits: limits.clone(),
        module: path.clone(),
      }));

      let mut null_error_handler = NullErrorHandler;

      // Require::Navigator navigator(navigationContext, nullErrorHandler);
      let mut navigator = Navigator::new(&mut navigation_context, &mut null_error_handler);

      // if (navigator.navigate(std::move(path)) != Status::Success) return std::nullopt;
      if navigator.navigate(path) != Status::Success {
        return None;
      }

      // The navigator borrows navigation_context for its lifetime; end that
      // borrow before reading back the (now-mutated) context.
      let _ = navigator;

      // if (!navigationContext.isModulePresent()) return std::nullopt;
      if !file_navigation_context_is_module_present(&navigation_context) {
        return None;
      }

      // if (std::optional<std::string> identifier = navigationContext.getIdentifier())
      //     return {{*identifier}};
      if let Some(identifier) = file_navigation_context_get_identifier(&navigation_context) {
        return Some(ModuleInfo {
          name: identifier,
          optional: false,
        });
      }

      None
    }
  }
}
