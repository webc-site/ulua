use alloc::{string::String, sync::Arc, vec::Vec};
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_table::AstExprTable, ast_local::AstLocal,
    ast_stat_local::AstStatLocal,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    first::first, follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, match_require::match_require,
    match_set_metatable::match_set_metatable, unwrap_group::unwrap_group,
  },
  records::{
    binding::Binding, count_mismatch::CountMismatchContext, metatable_type::MetatableType,
    scope::Scope, symbol::Symbol, table_type::TableType, type_checker::TypeChecker,
    type_fun::TypeFun,
  },
  type_aliases::{
    error_type::ErrorType, name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_local(
    &mut self,
    scope: &ScopePtr,
    local: &AstStatLocal,
  ) -> ControlFlow {
    let mut variable_types: Vec<TypeId> = Vec::new();
    let mut expected_types: Vec<Option<TypeId>> = Vec::new();
    let mut bindings: Vec<(*mut AstLocal, Binding)> = Vec::new();

    variable_types.reserve(local.vars.size);
    expected_types.reserve(local.vars.size);
    bindings.reserve(local.vars.size);

    let vars = local.vars.as_slice();
    let values = local.values.as_slice();

    for (i, &var) in vars.iter().enumerate() {
      // SAFETY: var 指向 AST arena 节点。
      let annotation = unsafe { (*var).annotation };
      let rhs_is_table = values
        .get(i)
        .and_then(|&val| ast_node_try_as::<AstExprTable>(unsafe { &(*val).base }))
        .is_some();

      let mut ty: TypeId = null_mut();
      if !annotation.is_null() {
        // SAFETY: annotation 非 null，指向 AST arena 节点。
        ty = self.resolve_type(scope.clone(), unsafe { &*annotation });
        if get_type_id::<ErrorType>(follow_type_id(ty)).is_some() {
          ty = null_mut();
        }
      }

      if ty.is_null() {
        ty = if rhs_is_table || !self.is_nonstrict_mode() {
          self.fresh_type_scope_ptr(scope.clone())
        } else {
          self.any_type
        };
      }

      variable_types.push(ty);
      expected_types.push(Some(ty));
      bindings.push((
        var,
        Binding {
          type_id: ty,
          // SAFETY: var 指向 AST arena 节点。
          location: unsafe { (*var).location },
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      ));
    }

    if !values.is_empty() {
      let variable_tail = self.fresh_type_pack_scope_ptr(scope.clone());
      let variable_pack = self
        .add_type_pack_vector_type_id_optional_type_pack_id(&variable_types, Some(variable_tail));
      let value_pack = self
        .check_expr_list(
          scope,
          &local.base.base.location,
          &local.values,
          true,
          &Vec::new(),
          &expected_types,
        )
        .r#type;

      let mut ctx = CountMismatchContext::ExprListResult;
      if let [first_val] = values {
        // SAFETY: values[0] 指向 AST arena 节点。
        let expr = unwrap_group(*first_val);
        if ast_node_try_as::<AstExprCall>(unsafe { &(*expr).base }).is_some() {
          ctx = CountMismatchContext::FunctionResult;
        }
      }

      self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
        value_pack,
        variable_pack,
        scope,
        &local.base.base.location,
        ctx,
      );

      if let (&[var], &[rhs]) = (vars, values)
        && let Some(ty) = first(value_pack, true)
      {
        if ast_node_try_as::<AstExprTable>(unsafe { &(*rhs).base }).is_some() {
          if let Some(ttv) = get_mutable_type_id::<TableType>(follow_type_id(ty))
            && ttv.name.is_none()
            && let Some(current_module) = self.current_module.as_ref()
          {
            let module_scope = current_module.get_module_scope();
            if Arc::ptr_eq(scope, &module_scope) {
              // SAFETY: var 指向 AST arena 节点。
              if !unsafe { (*var).name.value.is_null() } {
                // SAFETY: name.value 非 null，为 NUL 结尾 C 字符串。
                ttv.synthetic_name = Some(
                  unsafe { CStr::from_ptr((*var).name.value) }
                    .to_string_lossy()
                    .into_owned(),
                );
              }
            }
          }
        } else {
          let call = ast_node_try_as::<AstExprCall>(unsafe { &(*rhs).base });
          if call.is_some_and(match_set_metatable)
            && let Some(mtv) = get_mutable_type_id::<MetatableType>(follow_type_id(ty))
          {
            // SAFETY: var 指向 AST arena 节点。
            if !unsafe { (*var).name.value.is_null() } {
              // SAFETY: name.value 非 null，为 NUL 结尾 C 字符串。
              mtv.synthetic_name = Some(
                unsafe { CStr::from_ptr((*var).name.value) }
                  .to_string_lossy()
                  .into_owned(),
              );
            }
          }
        }
      }
    }

    // SAFETY: scope 为 Arc<Scope>，类型检查阶段单线程独占（C++ shared_ptr 可变访问同义）。
    let scope_mut = unsafe { &mut *(Arc::as_ptr(scope) as *mut Scope) };
    for (&var, &value) in vars.iter().zip(values.iter()) {
      let Some(call) = ast_node_try_as::<AstExprCall>(unsafe { &(*value).base }) else {
        continue;
      };

      let Some(require) = match_require(call) else {
        continue;
      };

      let Some(current_module) = self.current_module.as_ref() else {
        continue;
      };

      // SAFETY: resolver 由构造方保证有效（C++ 同契约）。
      let module_info = unsafe {
        ((*self.resolver).vtable.resolve_module_info)(self.resolver, &current_module.name, require)
      };

      let Some(module_info) = module_info else {
        continue;
      };

      // SAFETY: var 指向 AST arena 节点。
      if unsafe { (*var).name.value.is_null() } {
        continue;
      }

      // SAFETY: name.value 为 NUL 结尾 C 字符串。
      let name: Name = unsafe { CStr::from_ptr((*var).name.value) }
        .to_string_lossy()
        .into_owned();

      // SAFETY: 同 resolver 契约。
      unsafe {
        if let Some(module) = ((*self.resolver).vtable.get_module)(self.resolver, &module_info.name)
        {
          scope_mut
            .imported_type_bindings
            .insert(name.clone(), module.exported_type_bindings.clone());
          scope_mut
            .imported_modules
            .insert(name.clone(), module_info.name.clone());

          for require_cycle in &self.require_cycles {
            if !require_cycle.path.is_empty()
              && require_cycle.path[0] == module_info.name
              && let Some(imported_bindings) = scope_mut.imported_type_bindings.get_mut(&name)
            {
              for type_fun in imported_bindings.values_mut() {
                *type_fun = TypeFun::type_fun_type_id(self.any_type);
              }
            }
          }
        }
      }
    }

    for (local, binding) in bindings {
      scope_mut
        .bindings
        .insert(Symbol::from_local(local), binding);
    }

    ControlFlow::None
  }
}
