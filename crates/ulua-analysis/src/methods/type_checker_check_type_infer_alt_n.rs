use alloc::{string::String, sync::Arc};

use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::{
  enums::control_flow::ControlFlow,
  records::{binding::Binding, scope::Scope, symbol::Symbol, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_scope_ptr_type_id_scope_ptr_ast_stat_local_function(
    &mut self,
    scope: &ScopePtr,
    ty: TypeId,
    fun_scope: &ScopePtr,
    function: &AstStatLocalFunction,
  ) -> ControlFlow {
    // Name name = function.name->name.value; (declared in C++ parity, unused here)
    let _name = unsafe { (*function.name).name.value };

    unsafe {
      let scope_mut = Arc::as_ptr(scope) as *mut Scope;
      (*scope_mut).bindings.insert(
        Symbol::from_local(function.name),
        Binding {
          type_id: ty,
          location: function.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    self.check_function_body(fun_scope, ty, unsafe { &*function.func });

    let name_location = unsafe { (*function.name).location };
    let quantified = self.quantify(fun_scope, ty, name_location);
    unsafe {
      let scope_mut = Arc::as_ptr(scope) as *mut Scope;
      (*scope_mut).bindings.insert(
        Symbol::from_local(function.name),
        Binding {
          type_id: quantified,
          location: name_location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    ControlFlow::None
  }
}
