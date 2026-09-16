//! Node: `cxx:Function:Luau.Compiler:Compiler/src/ValueTracking.cpp:104:trackValues`
//!
//! `trackValues` — run a `ValueVisitor` over the AST root to record, for every
//! local, its initializer and whether it is ever reassigned (and which globals
//! are written). The C++ visitor holds references to `globals`/`variables`; the
//! Rust `ValueVisitor` owns them, so the constructor moves the maps in and we
//! move the populated results back out after the walk.

use ulua_ast::{
  records::{ast_local::AstLocal, ast_name::AstName, ast_node::AstNode},
  visit::dispatch_node,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  records::{value_visitor::ValueVisitor, variable::Variable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn track_values(
  globals: &mut DenseHashMap<AstName, Global>,
  variables: &mut DenseHashMap<*mut AstLocal, Variable>,
  class_locals: &mut DenseHashMap<AstName, *mut AstLocal>,
  root: *mut AstNode,
) {
  let mut visitor = ValueVisitor::new(globals, variables, class_locals);

  unsafe {
    dispatch_node(root, &mut visitor);
  }

  *globals = visitor.globals;
  *variables = visitor.variables;
  *class_locals = visitor.class_locals;
}
