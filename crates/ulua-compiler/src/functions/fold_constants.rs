use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_local::AstLocal,
    ast_name_table::AstNameTable, ast_node::AstNode,
  },
  visit::dispatch_node,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::table_constant_kind::TableConstantKind,
  records::{
    constant::Constant,
    constant_visitor::{ConstantVisitor, ConstantVisitorArgs},
    node::Node,
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
  pub constants: &'a mut DenseHashMap<Node<AstExpr>, Constant>,
  pub variables: &'a mut DenseHashMap<Node<AstLocal>, Variable>,
  pub locals: &'a mut DenseHashMap<Node<AstLocal>, Constant>,
  /// cpp 空指针语义的 Rust 化：无折叠表即 `None`
  pub builtins: Option<&'a DenseHashMap<Node<AstExprCall>, i32>>,
  pub fold_library_k: bool,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  pub string_table: &'a mut AstNameTable,
  pub table_constants: &'a DenseHashMap<Node<AstLocal>, TableConstantKind>,
  /// cpp 空指针语义的 Rust 化：不回滚变更记录即 `None`
  pub expr_change_log: Option<&'a mut ExprConstantChangeLog>,
  pub local_change_log: Option<&'a mut LocalConstantChangeLog>,
}

/// 对应 cpp `foldConstants`（cpp/Compiler/src/ConstantFolding.cpp:1295）：以
/// `ConstantVisitor` 对 `root` 子树跑一遍常量折叠，把结果回写进 `args` 各 map
/// （键均为该子树内节点的地址句柄）。`builtins`/change log 的 `Some`/`None` 即
/// cpp 传 nullptr 或实参的 Option 化，内容须与 root 子树一致。
pub fn fold_constants(root: &mut AstNode, args: FoldConstantsArgs<'_>) {
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

  // `dispatch_node` 以调用方独占的 `&mut root` 沿 parser 接线子指针遍历、折叠结果
  // 只写 map 不改写 AST 节点，`&mut visitor` 独占 args 中互不相交的借用，与 AST 无
  // 别名冲突。
  dispatch_node(root, &mut visitor);
}
