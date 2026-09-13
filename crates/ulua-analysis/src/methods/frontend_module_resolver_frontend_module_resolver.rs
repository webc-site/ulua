//! C++ `FrontendModuleResolver::FrontendModuleResolver(Frontend* frontend)`
//! (`Analysis/src/Frontend.cpp:1922`): stores the owning frontend; `modules`
//! and `moduleMutex` are default-initialized.
use std::{collections::HashMap, sync::Mutex};

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{
    frontend::Frontend,
    frontend_module_resolver::FrontendModuleResolver,
    module_info::ModuleInfo,
    module_resolver::{ModuleResolver, ModuleResolverVtable},
  },
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

impl FrontendModuleResolver {
  pub fn new(frontend: *mut Frontend) -> Self {
    Self {
      base: ModuleResolver {
        vtable: ModuleResolverVtable {
          resolve_module_info: frontend_module_resolver_resolve_module_info,
          get_module: frontend_module_resolver_get_module,
          module_exists: frontend_module_resolver_module_exists,
          get_human_readable_module_name: frontend_module_resolver_get_human_readable_module_name,
        },
      },
      frontend,
      module_mutex: Mutex::new(()),
      modules: HashMap::new(),
    }
  }
}

unsafe fn frontend_module_resolver_resolve_module_info(
  this: *mut ModuleResolver,
  current_module_name: &ModuleName,
  path_expr: *const AstExpr,
) -> Option<ModuleInfo> {
  if path_expr.is_null() {
    return None;
  }

  let resolver = this as *const FrontendModuleResolver;
  unsafe { (*resolver).resolve_module_info(current_module_name, &*path_expr) }
}

unsafe fn frontend_module_resolver_get_module(
  this: *const ModuleResolver,
  module_name: &ModuleName,
) -> Option<ModulePtr> {
  let resolver = this as *const FrontendModuleResolver;
  let _lock = unsafe { (*resolver).module_mutex.lock().unwrap() };
  unsafe { (*resolver).modules.get(module_name).cloned() }
}

unsafe fn frontend_module_resolver_module_exists(
  this: *const ModuleResolver,
  module_name: &ModuleName,
) -> bool {
  let resolver = this as *const FrontendModuleResolver;
  unsafe { (*resolver).module_exists(module_name) }
}

unsafe fn frontend_module_resolver_get_human_readable_module_name(
  this: *const ModuleResolver,
  module_name: &ModuleName,
) -> String {
  let resolver = this as *const FrontendModuleResolver;
  unsafe { (*resolver).get_human_readable_module_name(module_name) }
}
