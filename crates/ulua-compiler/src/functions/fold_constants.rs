use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_local::AstLocal,
    ast_name_table::AstNameTable, ast_node::AstNode,
  },
  visit::ast_node_visit,
};
use ulua_common::{
  FFlag::{LuauCompileFoldOptimize, LuauCompilePropagateTableProps2},
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::{
    table_constant_kind::TableConstantKind,
    type_constant_folding::Type::{Table, Unknown},
  },
  records::{
    constant::Constant,
    constant_visitor::{ConstantVisitor, ConstantVisitorArgs},
    table_mutation_tracker_deprecated::TableMutationTrackerDeprecated,
    variable::Variable,
  },
  type_aliases::{
    expr_constant_change_log::ExprConstantChangeLog,
    library_member_constant_callback::LibraryMemberConstantCallback,
    local_constant_change_log::LocalConstantChangeLog,
  },
};

#[derive(Debug)]
pub struct FoldConstantsArgs<'a> {
  pub constants: &'a mut DenseHashMap<*mut AstExpr, Constant>,
  pub variables: &'a mut DenseHashMap<*mut AstLocal, Variable>,
  pub locals: &'a mut DenseHashMap<*mut AstLocal, Constant>,
  pub builtins: *const DenseHashMap<*mut AstExprCall, i32>,
  pub fold_library_k: bool,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  pub string_table: &'a mut AstNameTable,
  pub table_constants: &'a DenseHashMap<*mut AstLocal, TableConstantKind>,
  pub expr_change_log: *mut ExprConstantChangeLog,
  pub local_change_log: *mut LocalConstantChangeLog,
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn fold_constants(root: *mut AstNode, args: FoldConstantsArgs<'_>) {
  let mut constant_tables_deprecated = DenseHashMap::new(null_mut::<AstLocal>());

  if LuauCompilePropagateTableProps2.get() && !LuauCompileFoldOptimize.get() {
    let mut mutation_tracker = TableMutationTrackerDeprecated {
      constant_tables: &mut constant_tables_deprecated,
      variables: args.variables,
    };
    unsafe {
      ast_node_visit(root, &mut mutation_tracker);
    }
  }

  let constant_tables_for_visitor = if LuauCompileFoldOptimize.get() {
    args.table_constants
  } else {
    &constant_tables_deprecated
  };

  let mut visitor = ConstantVisitor::new(ConstantVisitorArgs {
    constants: args.constants,
    variables: args.variables,
    locals: args.locals,
    builtins: args.builtins,
    fold_library_k: args.fold_library_k,
    library_member_constant_cb: args.library_member_constant_cb,
    string_table: args.string_table,
    constant_table_locals: constant_tables_for_visitor,
    expr_change_log: args.expr_change_log,
    local_change_log: args.local_change_log,
  });

  unsafe {
    ast_node_visit(root, &mut visitor);
  }

  if LuauCompilePropagateTableProps2.get() && !LuauCompileFoldOptimize.get() {
    for (_key, constant) in args.constants.iter_mut() {
      if constant.r#type == Table {
        constant.r#type = Unknown;
      }
    }

    for (_key, constant) in args.locals.iter_mut() {
      if constant.r#type == Table {
        constant.r#type = Unknown;
      }
    }
  }
}
