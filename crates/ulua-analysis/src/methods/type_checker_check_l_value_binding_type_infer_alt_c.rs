use alloc::{string::String, sync::Arc};
use core::ffi::CStr;

use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    binding::Binding,
    never_type::NeverType,
    scope::Scope,
    symbol::Symbol,
    type_checker::TypeChecker,
    type_error::TypeError,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_l_value_binding_scope_ptr_ast_expr_global(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprGlobal,
  ) -> TypeId {
    // SAFETY: expr.name.value 为 NUL 结尾 C 字符串（AST arena 持有）。
    let name: Name = unsafe {
      CStr::from_ptr(expr.name.value)
        .to_string_lossy()
        .into_owned()
    };
    let module_scope = self.current_module.as_ref().unwrap().get_module_scope();

    let sym = Symbol::from_global(expr.name);

    if let Some(binding) = module_scope.bindings.get(&sym) {
      let ty = follow_type_id(binding.type_id);
      if get_type_id::<NeverType>(ty).is_some() {
        return self.unknown_type;
      }
      return ty;
    }

    let result = self.fresh_type_scope_ptr(scope.clone());

    {
      let binding = Binding {
        type_id: result,
        location: expr.base.base.location,
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
      };
      // SAFETY: module_scope 是 module->scopes 共享的 Scope（C++ 经 shared_ptr
      // 可变访问同义）；类型检查阶段单线程独占，无并发别名。
      let module_scope_ptr = unsafe { &mut *(Arc::as_ptr(&module_scope) as *mut Scope) };
      module_scope_ptr.bindings.insert(sym, binding);
    }

    // If we're in strict mode, we want to report defining a global as an error,
    // but still add it to the bindings, so that autocomplete includes it in completions.
    if !self.is_nonstrict_mode() {
      let error_data = TypeErrorData::UnknownSymbol(UnknownSymbol::new(name, Context::Binding));
      let error =
        TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
      self.report_error_type_error(&error);
    }

    result
  }
}
