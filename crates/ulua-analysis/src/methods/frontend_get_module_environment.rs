use alloc::{string::String, sync::Arc};
use core::ptr::null;

use ulua_ast::records::location::Location;
use ulua_config::records::config::Config;

use crate::{
  records::{
    binding::Binding,
    frontend::Frontend,
    scope::Scope,
    scope_registry::{register_scope, resolve_scope_mut},
    source_module::SourceModule,
    symbol::Symbol,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl Frontend {
  pub fn get_module_environment(
    &self,
    module: &SourceModule,
    config: &Config,
    for_autocomplete: bool,
  ) -> ScopePtr {
    let mut result = if for_autocomplete {
      self.globals_for_autocomplete.global_scope.clone()
    } else {
      self.globals.global_scope.clone()
    };

    if let Some(environment_name) = &module.environment_name {
      result = self.get_environment_scope(environment_name.clone());
    }

    if !config.globals.is_empty() {
      let globals_scope = Arc::new(Scope::new(&result, 0));
      let globals_scope_id = register_scope(&globals_scope);
      result = globals_scope;

      for global in &config.globals {
        let name = module.names.get_str(global.as_str());

        if !name.is_null() {
          // 创建点写回经注册表写出口（注册表持 Arc 强引用保活，句柄必可
          // 解析；逐条 binding 顺序借用互不重叠，与 arc_as_mut 写穿同纪律）。
          let scope =
            resolve_scope_mut(globals_scope_id).expect("globals_scope_id 刚由 register_scope 发放");
          let binding = scope
            .bindings
            .entry(Symbol::from_global(name))
            .or_insert(Binding {
              type_id: null(),
              location: Location::default(),
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            });
          binding.type_id = self.builtin_types_ref().any_type;
        }
      }
    }

    result
  }
}
