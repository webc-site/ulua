//! Faithful port of `ControlFlow TypeChecker::check(const ScopePtr& scope, const AstStatFor& expr)`
//! (Analysis/src/TypeInfer.cpp:1175-1200).

use alloc::{string::String, sync::Arc};

use ulua_ast::records::ast_stat_for::AstStatFor;

use crate::{
  enums::control_flow::ControlFlow,
  records::{binding::Binding, scope::Scope, symbol::Symbol, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_for(
    &mut self,
    scope: &ScopePtr,
    expr: &AstStatFor,
  ) -> ControlFlow {
    // ScopePtr loopScope = childScope(scope, expr.location);
    let loop_scope = self.child_scope(scope, &expr.base.base.location);

    // TypeId loopVarType = number_type;
    let loop_var_type: TypeId = self.number_type;

    // if (expr.var->annotation)
    //     unify(loopVarType, resolveType(scope, *expr.var->annotation), scope, expr.location);
    let annotation = unsafe { (*expr.var).annotation };
    if !annotation.is_null() {
      let resolved = self.resolve_type(scope.clone(), unsafe { &*annotation });
      self.unify_type_id_type_id_scope_ptr_location(
        loop_var_type,
        resolved,
        scope,
        &expr.base.base.location,
      );
    }

    // loopScope->bindings[expr.var] = {loopVarType, expr.var->location};
    unsafe {
      let loop_scope_mut = Arc::as_ptr(&loop_scope) as *mut Scope;
      (*loop_scope_mut).bindings.insert(
        Symbol::from_local(expr.var),
        Binding {
          type_id: loop_var_type,
          location: (*expr.var).location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    // if (!expr.from)
    //     ice("Bad AstStatFor has no from expr");
    if expr.from.is_null() {
      self.ice_string("Bad AstStatFor has no from expr");
    }

    // if (!expr.to)
    //     ice("Bad AstStatFor has no to expr");
    if expr.to.is_null() {
      self.ice_string("Bad AstStatFor has no to expr");
    }

    // unify(checkExpr(loopScope, *expr.from).type, loopVarType, scope, expr.from->location);
    let from_ty = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        &loop_scope,
        unsafe { &*expr.from },
        None,
        false,
      )
      .r#type;
    self.unify_type_id_type_id_scope_ptr_location(from_ty, loop_var_type, scope, unsafe {
      &(*expr.from).base.location
    });

    // unify(checkExpr(loopScope, *expr.to).type, loopVarType, scope, expr.to->location);
    let to_ty = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        &loop_scope,
        unsafe { &*expr.to },
        None,
        false,
      )
      .r#type;
    self.unify_type_id_type_id_scope_ptr_location(to_ty, loop_var_type, scope, unsafe {
      &(*expr.to).base.location
    });

    // if (expr.step)
    //     unify(checkExpr(loopScope, *expr.step).type, loopVarType, scope, expr.step->location);
    if !expr.step.is_null() {
      let step_ty = self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          &loop_scope,
          unsafe { &*expr.step },
          None,
          false,
        )
        .r#type;
      self.unify_type_id_type_id_scope_ptr_location(step_ty, loop_var_type, scope, unsafe {
        &(*expr.step).base.location
      });
    }

    // check(loopScope, *expr.body);
    self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*expr.body });

    // return ControlFlow::None;
    ControlFlow::None
  }
}
