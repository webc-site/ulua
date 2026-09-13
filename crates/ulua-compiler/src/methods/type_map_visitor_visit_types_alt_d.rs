use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_for_in::AstStatForIn, ast_type::AstType,
  },
  rtti::ast_node_as,
  visit::{ast_expr_visit, ast_stat_visit},
};

use crate::{
  functions::is_matching_global::is_matching_global, records::type_map_visitor::TypeMapVisitor,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) fn visit_ast_stat_for_in(
  this: &mut TypeMapVisitor<'_>,
  node: *mut AstStatForIn,
) -> bool {
  unsafe {
    if node.is_null() {
      return false;
    }

    let node_ref = &*node;

    for &expr_ptr in node_ref.values.as_slice() {
      if !expr_ptr.is_null() {
        ast_expr_visit(expr_ptr, this);
      }
    }

    // This is similar to how Compiler matches builtin iteration, but we also handle generalized iteration case
    if node_ref.vars.len() == 2 && node_ref.values.len() == 1 {
      let value_ptr = node_ref.values.as_slice()[0];
      let call = ast_node_as::<AstExprCall>(value_ptr as *mut AstNode);

      if !call.is_null() && (*call).args.len() == 1 {
        let func = (*call).func;
        let arg = (*call).args.as_slice()[0];

        if is_matching_global(this.globals, func, "ipairs") {
          let indexer = this.try_get_table_indexer(arg);
          if !indexer.is_null() {
            this.record_resolved_type_ast_local_ast_type(
              node_ref.vars.as_slice()[0],
              &this.builtin_types.number_type as *const _ as *const AstType,
            );
            this.record_resolved_type_ast_local_ast_type(
              node_ref.vars.as_slice()[1],
              (*indexer).result_type,
            );
          }
        } else if is_matching_global(this.globals, func, "pairs") {
          let indexer = this.try_get_table_indexer(arg);
          if !indexer.is_null() {
            this.record_resolved_type_ast_local_ast_type(
              node_ref.vars.as_slice()[0],
              (*indexer).index_type,
            );
            this.record_resolved_type_ast_local_ast_type(
              node_ref.vars.as_slice()[1],
              (*indexer).result_type,
            );
          }
        }
      } else {
        let indexer = this.try_get_table_indexer(value_ptr);
        if !indexer.is_null() {
          this.record_resolved_type_ast_local_ast_type(
            node_ref.vars.as_slice()[0],
            (*indexer).index_type,
          );
          this.record_resolved_type_ast_local_ast_type(
            node_ref.vars.as_slice()[1],
            (*indexer).result_type,
          );
        }
      }
    }

    for &var_ptr in node_ref.vars.iter() {
      let var = &mut *var_ptr;

      if !var.annotation.is_null() {
        this.record_resolved_type_ast_local_ast_type(var_ptr, var.annotation);
      }
    }

    if !node_ref.body.is_null() {
      ast_stat_visit(node_ref.body as *mut AstStat, this);
    }
  }

  false
}

impl<'a> TypeMapVisitor<'a> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_for_in(&mut self, node: *mut AstStatForIn) -> bool {
    visit_ast_stat_for_in(self, node)
  }
}
