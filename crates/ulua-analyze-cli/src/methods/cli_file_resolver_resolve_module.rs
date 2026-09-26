use alloc::{boxed::Box, string::String, vec::Vec};

use ulua_analysis::{
  records::{module_info::ModuleInfo, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString},
  rtti::ast_node_try_as,
};
use ulua_require::{
  enums::status_require_navigator::Status,
  records::{error_handler::ErrorHandler, navigator::Navigator},
};

use crate::records::{
  cli_file_resolver::CliFileResolver, file_navigation_context::FileNavigationContext,
  luau_config_interrupt_info::LuauConfigInterruptInfo,
};

/// `Luau::Require::ErrorHandler nullErrorHandler{}` —— 空实现的错误处理器
/// (`CLI/src/Analyze.cpp:207`)。
struct NullErrorHandler;

impl ErrorHandler for NullErrorHandler {
  fn report_error(&mut self, _message: Vec<u8>) {}
}

impl CliFileResolver {
  /// C++ `std::optional<ModuleInfo> resolveModule(const ModuleInfo* context, AstExpr* node, const TypeCheckLimits& limits)`
  /// (`CLI/src/Analyze.cpp:185-221`).
  pub fn resolve_module(
    &mut self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    // if (AstExprConstantString* expr = node->as<AstExprConstantString>())
    let const_str = ast_node_try_as::<AstExprConstantString>(&expr.base)?;

    // std::string path{expr->value.data, expr->value.size};
    // 路径以原始字节进入导航器（cpp `std::string` 语义），判定链不做 UTF-8 校验
    let bytes: &[u8] = const_str.value.as_bytes();

    // FileNavigationContext navigationContext{context->name};
    // cpp 无条件解引用 context（空上下文为 UB），此处以空模块名兜底。
    let mut navigation_context =
      FileNavigationContext::new(context.map(|c| c.name.to_string()).unwrap_or_default());

    // LuauConfigInterruptInfo info = {limits, path};
    // navigationContext.luauConfigInit / luauConfigInterrupt capture &info.
    // `module` 只用于抛出取消错误（ulua-analysis 收 `String`），是链路唯一的展示转换点。
    navigation_context.interrupt_info = Some(Box::new(LuauConfigInterruptInfo {
      limits: limits.clone(),
      module: String::from_utf8_lossy(bytes).into_owned(),
    }));

    let mut null_error_handler = NullErrorHandler;

    // Require::Navigator navigator(navigationContext, nullErrorHandler);
    let mut navigator = Navigator::new(&mut navigation_context, &mut null_error_handler);

    // if (navigator.navigate(std::move(path)) != Status::Success) return std::nullopt;
    if navigator.navigate(bytes) != Status::Success {
      return None;
    }

    // The navigator borrows navigation_context for its lifetime; end that
    // borrow before reading back the (now-mutated) context.
    let _ = navigator;

    // if (!navigationContext.isModulePresent()) return std::nullopt;
    if !navigation_context.is_module_present() {
      return None;
    }

    // if (std::optional<std::string> identifier = navigationContext.getIdentifier())
    //     return {{*identifier}};
    navigation_context
      .get_identifier()
      .map(|identifier| ModuleInfo {
        name: ModuleName::from(identifier),
        optional: false,
      })
  }
}
