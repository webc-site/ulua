use alloc::vec::Vec;
use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_local::AstExprLocal, ast_stat_assign::AstStatAssign,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::FFlag;

use crate::{
  enums::{control_flow::ControlFlow, value_context::ValueContext},
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, begin_type_pack::begin, end_type_pack::end,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_mutable_type_pack::get_mutable_type_pack_id, get_table_type::get_table_type,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, is_generic::is_generic,
    is_nil::is_nil, maybe_generic::maybe_generic,
  },
  records::{
    free_type::FreeType, free_type_pack::FreeTypePack, function_type::FunctionType, symbol::Symbol,
    table_type::TableType, txn_log::TxnLog, type_checker::TypeChecker, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_pack_variant::TypePackVariant,
  },
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_assign(
    &mut self,
    scope: &ScopePtr,
    assign: &AstStatAssign,
  ) -> ControlFlow {
    let mut expected_types: Vec<Option<TypeId>> = Vec::with_capacity(assign.vars.size);

    let module_scope = self.current_module.as_ref().unwrap().get_module_scope();

    let vars = assign.vars.as_slice();

    for &dest in vars {
      // SAFETY: dest 指向 AST arena 节点。
      let dest_node = unsafe { &(*dest).base };

      if let Some(a_local) = ast_node_try_as::<AstExprLocal>(dest_node) {
        // AstExprLocal l-values will have to be checked again because their type
        // might have been mutated during checkExprList later
        expected_types.push(scope.lookup_symbol(Symbol::from_local(a_local.local)));
      } else if let Some(a_global) = ast_node_try_as::<AstExprGlobal>(dest_node) {
        // AstExprGlobal l-values lookup is inlined here to avoid creating a global
        // binding before checkExprList
        match module_scope
          .bindings
          .get(&Symbol::from_global(a_global.name))
        {
          Some(binding) => expected_types.push(Some(binding.type_id)),
          None => expected_types.push(None),
        }
      } else {
        // SAFETY: dest 指向 AST arena 节点。
        expected_types.push(Some(self.check_l_value(
          scope,
          unsafe { &*dest },
          ValueContext::LValue,
        )));
      }
    }

    let value_pack = self
      .check_expr_list(
        scope,
        &assign.base.base.location,
        &assign.values,
        false,
        &Vec::new(),
        &expected_types,
      )
      .r#type;

    let mut value_iter = begin(value_pack);
    let value_end = end(value_pack);

    let mut growing_pack: Option<&'static mut TypePack> = None;

    let values = assign.values.as_slice();

    for (i, &dest) in vars.iter().enumerate() {
      // SAFETY: dest 指向 AST arena 节点。
      let dest_ref = unsafe { &*dest };
      let is_local = ast_node_is::<AstExprLocal>(dest_ref);
      let is_global = ast_node_is::<AstExprGlobal>(dest_ref);

      let left: TypeId = if is_local || is_global {
        self.check_l_value(scope, dest_ref, ValueContext::LValue)
      } else {
        expected_types[i].unwrap()
      };

      let mut right: TypeId = null();

      let loc = values
        .get(i)
        .or_else(|| values.last())
        .map(|&val| unsafe { (*val).base.location })
        .unwrap_or(assign.base.base.location);

      if value_iter.operator_ne(&value_end) {
        right = follow_type_id(*value_iter.operator_deref());
        value_iter.operator_inc();
      } else if let Some(growing) = growing_pack.as_deref_mut() {
        growing.head.push(left);
        continue;
      } else if let Some(tail) = value_iter.tail() {
        // SAFETY: follow_type_pack_id 为 unsafe 函数，句柄有效。
        let tail_pack = unsafe { follow_type_pack_id(tail) };
        if get_type_pack_id::<ErrorTypePack>(tail_pack).is_some() {
          right = self.error_recovery_type_scope_ptr(scope);
        } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tail_pack) {
          right = vtp.ty;
        } else if get_type_pack_id::<FreeTypePack>(tail_pack).is_some() {
          // SAFETY: as_mutable_type_pack_id 去 const（C++ asMutable 同义），句柄有效。
          unsafe {
            (*as_mutable_type_pack_id(tail_pack)).ty = TypePackVariant::TypePack(TypePack {
              head: alloc::vec![left],
              tail: None,
            });
          }
          growing_pack = get_mutable_type_pack_id::<TypePack>(tail_pack);
        }
      }

      if !right.is_null() {
        if !FFlag::LuauInstantiateInSubtyping.get() && !maybe_generic(left) && is_generic(right) {
          right = self.instantiate(scope, right, loc, TxnLog::empty());
        }

        // Setting a table entry to nil doesn't mean nil is the type of the indexer,
        // it is just deleting the entry
        let mut dest_table_type_receiving_nil: Option<&TableType> = None;
        // SAFETY: repr(C) base 偏移 0，cast 有效。
        let index_expr = ast_node_try_as::<AstExprIndexExpr>(unsafe { &(*dest).base });
        if is_nil(right)
          && let Some(index_expr) = index_expr
        {
          let expr_ty = self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
              scope,
              // SAFETY: index_expr->expr 指向 AST arena 节点。
              unsafe { &*index_expr.expr },
              None,
              false,
            )
            .r#type;
          dest_table_type_receiving_nil = get_table_type(expr_ty);
        }

        if dest_table_type_receiving_nil.is_none_or(|t| t.indexer.is_none()) {
          // In nonstrict mode, any assignments where the lhs is free and rhs isn't
          // a function, we give it any type.
          if self.is_nonstrict_mode()
            && get_type_id::<FreeType>(follow_type_id(left)).is_some()
            && get_type_id::<FunctionType>(follow_type_id(right)).is_none()
          {
            self.unify_type_id_type_id_scope_ptr_location(self.any_type, left, scope, &loc);
          } else {
            self.unify_type_id_type_id_scope_ptr_location(right, left, scope, &loc);
          }
        }
      }
    }

    ControlFlow::None
  }
}
