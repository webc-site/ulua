use core::ffi::c_void;
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use ulua_ast::records::{ast_expr_table::AstExprTable, ast_name::AstName};
use ulua_common::records::dense_hash_pointer::DenseHashPointer;

pub fn shape_visitor_hasher_operator_call(p: (*mut AstExprTable, AstName)) -> usize {
  let mut state = DefaultHasher::new();
  p.1.hash(&mut state);
  let name_hash = state.finish() as usize;
  DenseHashPointer.call(p.0 as *const c_void) ^ name_hash
}
