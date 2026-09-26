//! C++ `Luau::ModuleResolver` 纯虚基类（`Analysis/include/Luau/ModuleResolver.h:18`）
//! 的 Rust 化：虚函数接口 → trait；接口值在检查管线中以 [`ModuleResolverRef`]
//! 具体分派枚举（`Handle` 薄句柄 + match 静态分派）承载，取代
//! `&mut dyn ModuleResolver` / `*mut dyn ModuleResolver` 的 vtable 间接调用。
//!
//! C++ 侧 4 个方法均为纯虚，故 trait 无默认实现；具体 resolver
//! （[`FrontendModuleResolver`](crate::records::frontend_module_resolver::FrontendModuleResolver)
//! / [`NullModuleResolver`](crate::records::null_module_resolver::NullModuleResolver)）
//! 各自实现。

use alloc::string::String;
use core::fmt::{self, Debug};

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{
    arena_handle::Handle, frontend_module_resolver::FrontendModuleResolver,
    module_info::ModuleInfo, null_module_resolver::NullModuleResolver,
  },
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

/// `ModuleResolver` 接口的具体分派句柄：取代此前在约束生成器/求解器与
/// `TypeChecker` 之间以 `*mut dyn ModuleResolver` 传递的 trait object。
///
/// 实现方集合是全仓（含 cpp oracle）封闭的两种——`FrontendModuleResolver`
/// （`Frontend` 的两个具体字段）与 `NullModuleResolver`（`parseType` 局部）——
/// 无任何宿主注入点，故用 [`Handle`] 薄句柄枚举做 match 静态分派：去掉
/// vtable 间接与 fat-pointer 存储，require 解析热路径（`get_module`/
/// `module_exists`）可内联。句柄即 cpp `ModuleResolver*`（NotNull 别名）的
/// 对应物，存活期契约由 [`Handle`] 的类型级文档统一约束。
#[derive(Clone, Copy)]
pub enum ModuleResolverRef {
  /// cpp `Frontend` 持有的 `FrontendModuleResolver`（正式检查管线）。
  Frontend(Handle<FrontendModuleResolver>),
  /// cpp `parseTypeContext` 的 `NullModuleResolver`（一切查询回答未知）。
  Null(Handle<NullModuleResolver>),
}

/// [`ModuleResolverRef::resolve`] 的借用视图（私有；引用必然有效）。
enum ModuleResolverRefTarget<'a> {
  Frontend(&'a FrontendModuleResolver),
  Null(&'a NullModuleResolver),
}

impl ModuleResolverRef {
  /// 唯一的解引用收口点：按实现方转发到具体类型（静态分派，无 vtable）。
  ///
  /// 两个变体只能由下方 `From<&mut _>` 构造（`Handle` 编码非空与存活
  /// 契约，见 [`Handle`] 模块文档），故此处直接 `get()` 物化共享引用。
  fn resolve(&self) -> ModuleResolverRefTarget<'_> {
    match self {
      Self::Frontend(resolver) => ModuleResolverRefTarget::Frontend(resolver.get()),
      Self::Null(resolver) => ModuleResolverRefTarget::Null(resolver.get()),
    }
  }

  /// 转发 [`ModuleResolver::resolve_module_info`]。固有方法（非 trait
  /// 方法）：分派枚举是封闭集合的最终消费者，调用点无需 import trait。
  pub fn resolve_module_info(
    &self,
    current_module_name: &ModuleName,
    path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    match self.resolve() {
      ModuleResolverRefTarget::Frontend(resolver) => {
        ModuleResolver::resolve_module_info(resolver, current_module_name, path_expr)
      }
      ModuleResolverRefTarget::Null(resolver) => {
        ModuleResolver::resolve_module_info(resolver, current_module_name, path_expr)
      }
    }
  }

  /// 转发 [`ModuleResolver::get_module`]。
  pub fn get_module(&self, module_name: &ModuleName) -> Option<ModulePtr> {
    match self.resolve() {
      ModuleResolverRefTarget::Frontend(resolver) => {
        ModuleResolver::get_module(resolver, module_name)
      }
      ModuleResolverRefTarget::Null(resolver) => ModuleResolver::get_module(resolver, module_name),
    }
  }

  /// 转发 [`ModuleResolver::module_exists`]。
  pub fn module_exists(&self, module_name: &ModuleName) -> bool {
    match self.resolve() {
      ModuleResolverRefTarget::Frontend(resolver) => {
        ModuleResolver::module_exists(resolver, module_name)
      }
      ModuleResolverRefTarget::Null(resolver) => {
        ModuleResolver::module_exists(resolver, module_name)
      }
    }
  }

  /// 转发 [`ModuleResolver::get_human_readable_module_name`]。
  pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    match self.resolve() {
      ModuleResolverRefTarget::Frontend(resolver) => {
        ModuleResolver::get_human_readable_module_name(resolver, module_name)
      }
      ModuleResolverRefTarget::Null(resolver) => {
        ModuleResolver::get_human_readable_module_name(resolver, module_name)
      }
    }
  }
}

impl From<&mut FrontendModuleResolver> for ModuleResolverRef {
  fn from(resolver: &mut FrontendModuleResolver) -> Self {
    Self::Frontend(Handle::from_mut(resolver))
  }
}

impl From<&mut NullModuleResolver> for ModuleResolverRef {
  fn from(resolver: &mut NullModuleResolver) -> Self {
    Self::Null(Handle::from_mut(resolver))
  }
}

impl fmt::Debug for ModuleResolverRef {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.resolve() {
      ModuleResolverRefTarget::Frontend(resolver) => Debug::fmt(resolver, f),
      ModuleResolverRefTarget::Null(resolver) => Debug::fmt(resolver, f),
    }
  }
}
