use alloc::{string::String, sync::Arc};

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{
    module_info::ModuleInfo, require_suggester::RequireSuggester, source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};

/// C++ `Luau::FileResolver` 虚基类（`Analysis/include/Luau/FileResolver.h`）的
/// Rust 化：虚函数接口 → trait，实现方以 `dyn FileResolver`（trait object）
/// 传递，替代原先 `#[repr(C)]` 手写 vtable + `unsafe fn` 指针。
///
/// 默认方法体对应 C++ 基类的缺省实现；`readSource` 为纯虚，无默认。
/// C++ 成员 `std::shared_ptr<RequireSuggester> requireSuggester` 由实现方
/// 自行持有，经 [`FileResolver::require_suggester`] 读取（默认不挂）。
pub trait FileResolver {
  /// `readSource`：读取模块源码；C++ 纯虚。
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode>;

  /// `resolveModule`：解析 require 表达式指向的模块；基类缺省返回 `nullopt`。
  ///
  /// C++ 的 `const ModuleInfo* context` 可空（首层 require 无上下文），此处用
  /// `Option<&ModuleInfo>` 表达；`AstExpr* expr` 由调用方保证非空，用 `&AstExpr`。
  fn resolve_module(
    &mut self,
    _context: Option<&ModuleInfo>,
    _expr: &AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    None
  }

  /// `getHumanReadableModuleName`：基类缺省原样返回模块名。
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    name.clone()
  }

  /// `getEnvironmentForModule`：基类缺省无每模块环境映射。
  fn get_environment_for_module(&self, _name: &ModuleName) -> Option<String> {
    None
  }

  /// C++ 成员 `requireSuggester` 的读取入口；基类缺省为空。
  fn require_suggester(&self) -> Option<&Arc<dyn RequireSuggester>> {
    None
  }
}
