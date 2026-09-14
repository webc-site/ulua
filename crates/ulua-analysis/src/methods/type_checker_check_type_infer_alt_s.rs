use alloc::{string::String, sync::Arc};
use core::ffi::CStr;

use ulua_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

use crate::{
  enums::control_flow::ControlFlow,
  records::{
    binding::Binding, module::Module, scope::Scope, symbol::Symbol, type_checker::TypeChecker,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};

impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_declare_global(
    &mut self,
    scope: &ScopePtr,
    global: &AstStatDeclareGlobal,
  ) -> ControlFlow {
    let global_ty = if global.type_.is_null() {
      self.error_recovery_type_scope_ptr(scope)
    } else {
      self.resolve_type(scope.clone(), unsafe { &*global.type_ })
    };
    let global_name = unsafe { CStr::from_ptr(global.name.value) }
      .to_string_lossy()
      .into_owned();

    unsafe {
      let module =
        Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
      (*module).declared_globals.insert(global_name, global_ty);

      let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
      (*scope_raw).bindings.insert(
        Symbol::from_global(global.name),
        Binding {
          type_id: global_ty,
          location: global.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    ControlFlow::None
  }
}
