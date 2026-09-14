use alloc::{string::String, sync::Arc};
use core::{ffi::CStr, ptr::null};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::table_state::TableState,
  functions::get_mutable_table_type::get_mutable_table_type,
  records::{
    binding::Binding, scope::Scope, symbol::Symbol, type_checker::TypeChecker,
    type_level::TypeLevel,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  // Answers the question: "Can I define another function with this name?"
  // Primarily about detecting duplicates.
  pub fn check_function_name(
    &mut self,
    scope: &ScopePtr,
    fun_name: &AstExpr,
    level: TypeLevel,
  ) -> TypeId {
    // auto freshTy = [&]() { return freshType(level); };
    let node = &fun_name.base;

    if let Some(global_name) = ast_node_try_as::<AstExprGlobal>(node) {
      let module_scope = self.current_module.as_ref().unwrap().get_module_scope();
      let name = Symbol::from_global(global_name.name);
      if module_scope.bindings.contains_key(&name) {
        if self.is_nonstrict_mode() {
          return module_scope.bindings.get(&name).unwrap().type_id;
        }

        return self.error_recovery_type_scope_ptr(scope);
      } else {
        let ty = self.fresh_type_type_level(level);
        let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;
        let binding = Binding {
          type_id: ty,
          location: fun_name.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        };
        unsafe {
          (*module_scope_ptr).bindings.insert(name, binding);
        }
        return ty;
      }
    }

    if let Some(local_name) = ast_node_try_as::<AstExprLocal>(node) {
      let name = Symbol::from_local(local_name.local);
      let scope_ptr = Arc::as_ptr(scope) as *mut Scope;
      // Binding& binding = scope->bindings[name];  — default-constructs (typeId == nullptr) if absent.
      let binding = unsafe {
        (*scope_ptr)
          .bindings
          .entry(name)
          .or_insert_with(|| Binding {
            type_id: null(),
            location: fun_name.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          })
      };
      if binding.type_id.is_null() {
        *binding = Binding {
          type_id: self.fresh_type_type_level(level),
          location: fun_name.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        };
      }
      return binding.type_id;
    }

    if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(node) {
      let lhs_type = self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          scope,
          unsafe { &*index_name.expr },
          None,
          false,
        )
        .r#type;
      let ttv = get_mutable_table_type(lhs_type);

      // C++ TypeChecker.cpp（checkFunctionName）: if (!ttv || ttv->state == TableState::Sealed)
      if ttv.as_ref().is_none_or(|t| t.state == TableState::Sealed) {
        let name: Name = unsafe {
          CStr::from_ptr(index_name.index.value)
            .to_string_lossy()
            .into_owned()
        };
        if let Some(ty) = self.get_index_type_from_type(
          scope.clone(),
          lhs_type,
          &name,
          &index_name.index_location,
          false,
        ) {
          return ty;
        }

        return self.error_recovery_type_scope_ptr(scope);
      }

      let name: Name = unsafe {
        CStr::from_ptr(index_name.index.value)
          .to_string_lossy()
          .into_owned()
      };

      // 上面分支全部 return，走到这里 ttv 必为 Some 且非 Sealed。
      let ttv = ttv.unwrap();

      if ttv.props.contains_key(&name) {
        return ttv.props.get(&name).unwrap().type_deprecated();
      }

      let fresh = self.fresh_type_type_level(level);
      let property = ttv.props.entry(name).or_default();
      property.set_type(fresh);
      property.location = Some(index_name.index_location);
      return property.type_deprecated();
    }

    if ast_node_try_as::<AstExprError>(node).is_some() {
      return self.error_recovery_type_scope_ptr(scope);
    }

    self.ice_string_location("Unexpected AST node type", &fun_name.base.location);
    self.error_recovery_type_scope_ptr(scope)
  }
}
