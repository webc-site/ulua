use ulua_ast::{
  records::{ast_expr_table::AstExprTable, ast_local::AstLocal, ast_node::AstNode},
  visit::ast_node_visit,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::table_constant_kind::TableConstantKind,
  functions::unwrap_expr_of_type::unwrap_expr_of_type,
  records::{
    table_mutation_tracker::TableMutationTracker,
    table_mutation_tracker_deprecated::TableMutationTrackerDeprecated, variable::Variable,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn build_table_constant_map(
  result: &mut DenseHashMap<*mut AstLocal, TableConstantKind>,
  variables: &DenseHashMap<*mut AstLocal, Variable>,
  root: *mut AstNode,
) {
  LUAU_ASSERT!(
    FFlag::LuauCompileFoldOptimize.get() && FFlag::LuauCompilePropagateTableProps2.get()
  );

  if FFlag::LuauCompileNewTableMutationTracker.get() {
    let mut tracker = TableMutationTracker::new(variables);
    unsafe {
      ast_node_visit(root, &mut tracker);
    }

    for (local, var) in variables.iter() {
      if var.written {
        continue;
      }

      if var.init.is_null() || unwrap_expr_of_type::<AstExprTable>(var.init).is_null() {
        continue;
      }

      if !tracker.escaped.contains(local) {
        *result.get_or_insert(*local) = TableConstantKind::ConstantTable;
      }
    }
  } else {
    let mut mutation_tracker = TableMutationTrackerDeprecated::new(result, variables);
    unsafe {
      ast_node_visit(root, &mut mutation_tracker);
    }
  }
}
