use alloc::{sync::Arc, vec::Vec};

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_varargs::AstExprVarargs,
    location::Location,
  },
  rtti::ast_node_is,
};
use ulua_common::FFlag;

use crate::{
  functions::{
    contains_never::contains_never, first::first, follow_type::follow_type_id,
    get_mutable_type_pack::get_mutable_type_pack_id, get_type_alt_j::get_type_id,
  },
  records::{
    module::Module, never_type::NeverType, txn_log::TxnLog, type_checker::TypeChecker,
    type_pack::TypePack, type_pack_var::TypePackVar, with_predicate::WithPredicate,
  },
  type_aliases::{
    predicate_vec::PredicateVec, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub fn check_expr_list(
    &mut self,
    scope: &ScopePtr,
    location: &Location,
    exprs: &AstArray<*mut AstExpr>,
    substitute_free_for_nil: bool,
    instantiate_generics: &[bool],
    expected_types: &[Option<TypeId>],
  ) -> WithPredicate<TypePackId> {
    let mut uninhabitable = false;
    let pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
      head: Vec::new(),
      tail: None,
    }));
    let mut predicates: PredicateVec = Vec::new(); // At the moment we will be pushing all predicate sets into this. Do we need some way to split them up?

    let expr_slice = exprs.as_slice();
    if expr_slice.is_empty() {
      return WithPredicate::with_predicate_t(pack);
    }

    // pack 刚以 TypePack 变体分配，下转必然成功。
    let tp = get_mutable_type_pack_id::<TypePack>(pack).unwrap();

    let last_index = expr_slice.len().saturating_sub(1);
    tp.head.reserve(expr_slice.len());

    let mut state = self.mk_unifier(scope, location);

    let mut inverse_logs: Vec<TxnLog> = Vec::new();

    // SAFETY: current_module 在类型检查期间独占使用（C++ 直接改 module->astTypes 同义）。
    let module =
      unsafe { &mut *(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module) };

    for (i, &expr) in expr_slice.iter().enumerate() {
      let expected_type: Option<TypeId> = expected_types.get(i).copied().flatten();

      // SAFETY: expr 指向 AST arena 节点。
      let expr_ref = unsafe { &*expr };
      let is_call_or_varargs =
        ast_node_is::<AstExprCall>(expr_ref) || ast_node_is::<AstExprVarargs>(expr_ref);

      if i == last_index && is_call_or_varargs {
        let result = self.check_expr_pack(scope, expr_ref);
        let type_pack = result.r#type;
        predicates.extend(result.predicates);

        if contains_never(type_pack) {
          // f(), g() where f() returns (never, string) or (string, never) means this whole TypePackId is uninhabitable, so return (never,
          // ...never)
          uninhabitable = true;
          continue;
        } else if let Some(first_ty) = first(type_pack, true) {
          let key = expr as *const AstExpr;
          if module.ast_types.find(&key).is_none() {
            *module.ast_types.get_or_insert(key) = follow_type_id(first_ty);
          }
        }

        if let Some(expected_type) = expected_type {
          let key = expr as *const AstExpr;
          *module.ast_expected_types.get_or_insert(key) = expected_type;
        }

        tp.tail = Some(type_pack);
      } else {
        let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          scope,
          expr_ref,
          expected_type,
          false,
        );
        let r#type = result.r#type;
        predicates.extend(result.predicates);

        if get_type_id::<NeverType>(r#type).is_some() {
          // f(), g() where f() returns (never, string) or (string, never) means this whole TypePackId is uninhabitable, so return (never,
          // ...never)
          uninhabitable = true;
          continue;
        }

        let is_constant_nil = ast_node_is::<AstExprConstantNil>(expr_ref);
        let mut actual_type = if substitute_free_for_nil && is_constant_nil {
          self.fresh_type_scope_ptr(scope.clone())
        } else {
          r#type
        };

        if !FFlag::LuauInstantiateInSubtyping.get()
          && instantiate_generics.get(i).copied().unwrap_or(false)
        {
          let loc = expr_ref.base.location;
          actual_type = self.instantiate(scope, actual_type, loc, TxnLog::empty());
        }

        if let Some(expected_type) = expected_type {
          state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
            actual_type,
            expected_type,
            false,
            false,
            None,
          );

          // Ugly: In future iterations of the loop, we might need the state of the unification we
          // just performed. There's not a great way to pass that into checkExpr. Instead, we store
          // the inverse of the current log, and commit it. When we're done, we'll commit all the
          // inverses. This isn't optimal, and a better solution is welcome here.
          inverse_logs.push(state.log.inverse());
          state.log.commit();
        }

        tp.head.push(actual_type);
      }
    }

    for log in &mut inverse_logs {
      log.commit();
    }

    if uninhabitable {
      return WithPredicate::with_predicate_t(self.uninhabitable_type_pack);
    }
    WithPredicate::with_predicate_t_predicate_vec(pack, predicates)
  }
}
