use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_local::AstLocal,
    ast_name_table::AstNameTable, ast_node::AstNode,
  },
  visit::ast_node_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::table_constant_kind::TableConstantKind,
  records::{
    constant::Constant,
    constant_visitor::{ConstantVisitor, ConstantVisitorArgs},
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
  // cpp/Compiler/src/ConstantFolding.cpp:1295 `foldConstants` 只跑一遍 ConstantVisitor；
  // 旧 `LuauCompileFoldOptimize=false` 分支（deprecated tracker 预处理 + 事后把 Table
  // 常量退回 Unknown）在上游已无对应实现，随之移除。
  let mut visitor = ConstantVisitor::new(ConstantVisitorArgs {
    constants: args.constants,
    variables: args.variables,
    locals: args.locals,
    builtins: args.builtins,
    fold_library_k: args.fold_library_k,
    library_member_constant_cb: args.library_member_constant_cb,
    string_table: args.string_table,
    constant_table_locals: args.table_constants,
    expr_change_log: args.expr_change_log,
    local_change_log: args.local_change_log,
  });

  unsafe {
    ast_node_visit(root, &mut visitor);
  }
}
