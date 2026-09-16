use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_local::AstLocal, ast_name::AstName,
    ast_name_table::AstNameTable, ast_node::AstNode,
  },
  visit::ast_node_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  records::{builtin_visitor::BuiltinVisitor, compile_options::CompileOptions, variable::Variable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn analyze_builtins(
  result: &mut DenseHashMap<*mut AstExprCall, i32>,
  globals: &DenseHashMap<AstName, Global>,
  variables: &DenseHashMap<*mut AstLocal, Variable>,
  options: &CompileOptions,
  root: *mut AstNode,
  names: &AstNameTable,
) {
  let mut visitor = BuiltinVisitor::new(result, globals, variables, options, names);
  // C++ `root->visit(&visitor)` — traverse the WHOLE tree so the visitor's
  // visit_expr_call fires for every call. The model instead cast `root` (an
  // AstStatBlock!) to AstExprCall and called `visit` once, registering nothing
  // -> all optimization-level-2 builtin constant-folding silently no-op'd.
  unsafe {
    ast_node_visit(root, &mut visitor);
  }
}
