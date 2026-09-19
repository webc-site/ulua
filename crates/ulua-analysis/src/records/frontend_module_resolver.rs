//! Source: `Analysis/include/Luau/Frontend.h`
//!
//! C++ `struct FrontendModuleResolver : ModuleResolver` (Frontend.h:133-151).
//! C++ 基类的纯虚接口在 Rust 侧由 [`ModuleResolver`] trait 承接，数据成员移植
//! 到本结构；接口的具体实现见下方 `impl ModuleResolver`（转发到同名固有方法）。

use alloc::string::String;
use core::fmt::{Debug, Formatter, Result};
use std::{collections::HashMap, sync::Mutex};

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{frontend::Frontend, module_info::ModuleInfo, module_resolver::ModuleResolver},
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

pub struct FrontendModuleResolver {
  /// `Frontend* frontend;`
  pub frontend: *mut Frontend,

  /// `mutable std::mutex moduleMutex;`
  pub module_mutex: Mutex<()>,

  /// `std::unordered_map<ModuleName, ModulePtr> modules;`
  pub modules: HashMap<ModuleName, ModulePtr>,
}

/// 基类接口 → 固有方法转发；`Self::` 路径按语言规则优先解析到固有实现。
impl ModuleResolver for FrontendModuleResolver {
  fn resolve_module_info(
    &self,
    current_module_name: &ModuleName,
    path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    Self::resolve_module_info(self, current_module_name, path_expr)
  }

  fn get_module(&self, module_name: &ModuleName) -> Option<ModulePtr> {
    self.try_get_module(module_name)
  }

  fn module_exists(&self, module_name: &ModuleName) -> bool {
    Self::module_exists(self, module_name)
  }

  fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    Self::get_human_readable_module_name(self, module_name)
  }
}

impl Debug for FrontendModuleResolver {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FrontendModuleResolver")
      .field("modules", &self.modules)
      .finish_non_exhaustive()
  }
}
