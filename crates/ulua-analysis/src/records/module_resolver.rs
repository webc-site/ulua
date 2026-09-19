//! C++ `Luau::ModuleResolver` 纯虚基类（`Analysis/include/Luau/ModuleResolver.h:18`）
//! 的 Rust 化：虚函数接口 → trait，实现方以 `*mut dyn ModuleResolver`（trait
//! object）传递，取代原先 `#[repr(C)]` 手写 vtable + `unsafe fn` 指针（与
//! [`crate::records::file_resolver::FileResolver`] 同一模式）。
//!
//! C++ 侧 4 个方法均为纯虚，故 trait 无默认实现；具体 resolver
//! （`FrontendModuleResolver` / `NullModuleResolver`）各自实现。

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::module_info::ModuleInfo,
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

pub trait ModuleResolver {
  /// `resolveModuleInfo`：由 `require()` 的 AST 实参推出 `ModuleInfo`；
  /// 返回 `None` 表示无法判定（C++ 同语义）。`path_expr` 对应 C++ 的
  /// `const AstExpr&`，故用不可空引用。
  fn resolve_module_info(
    &self,
    current_module_name: &ModuleName,
    path_expr: &AstExpr,
  ) -> Option<ModuleInfo>;

  /// `getModule`：编译期未知或类型检查中途成环时为 `None`（C++ `nullptr`）。
  fn get_module(&self, module_name: &ModuleName) -> Option<ModulePtr>;

  /// `moduleExists`：区分 `get_module` 返回 `None` 的两种原因。
  fn module_exists(&self, module_name: &ModuleName) -> bool;

  /// `getHumanReadableModuleName`：错误信息用的可读模块名。
  fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String;
}
