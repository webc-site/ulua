use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_local::AstLocal, ast_name::AstName,
    ast_name_table::AstNameTable, ast_node::AstNode,
  },
  visit::dispatch_node,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  records::{
    builtin_visitor::BuiltinVisitor, compile_options::CompileOptions, node::Node,
    variable::Variable,
  },
};

/// 对应 cpp `analyzeBuiltins`（cpp/Compiler/src/Builtins.cpp:468）：以 `BuiltinVisitor`
/// 遍历整棵 AST，收集所有可折叠的内建 `AstExprCall`（以节点裸指针为地址键）。
/// `root` 为编译入口的 `AstStatBlock`（向上转 `AstNode`），`variables` 的
/// `*mut AstLocal` 键均来自先行 `track_values` 对同一棵树的登记。
pub(crate) fn analyze_builtins(
  globals: &DenseHashMap<AstName, Global>,
  variables: &DenseHashMap<Node<AstLocal>, Variable>,
  options: &CompileOptions,
  root: &mut AstNode,
  names: &AstNameTable,
) -> DenseHashMap<Node<AstExprCall>, i32> {
  let mut result = DenseHashMap::default();
  {
    let mut visitor = BuiltinVisitor::new(&mut result, globals, variables, options, names);
    // 对应 C++ `root->visit(&visitor)`——遍历整棵树，让 visitor 的
    // visit_expr_call 对每个调用点都触发。旧模型曾把 `root`（其实是
    // AstStatBlock!）强转成 AstExprCall 后只 `visit` 一次，什么都没登记
    // —— 优化等级 2 的内建常量折叠就此整体静默失效。
    // `dispatch_node` 收的是调用方持有独占借用的 `&mut root`（repr(C) arena 节点，编译
    // 结束前 arena 由 parse 入口持有、地址稳定）；它只沿 parser 接线的子指针遍历，对
    // AST 仅短读、不另构造 `&mut`，与本借用不构成写写别名。
    dispatch_node(root, &mut visitor);
  }
  result
}
