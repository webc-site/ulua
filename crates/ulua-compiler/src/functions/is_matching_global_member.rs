use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_name::AstName,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{enums::global::Global, functions::get_global_state::get_global_state};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn is_matching_global_member(
  globals: &DenseHashMap<AstName, Global>,
  expr: *mut AstExprIndexName,
  library: &str,
  member: &str,
) -> bool {
  let expr_global = unsafe {
    let expr_ptr = expr;
    let obj = (*expr_ptr).expr;
    ast_node_as::<AstExprGlobal>(obj as *mut AstNode)
  };

  if !expr_global.is_null() {
    let object = unsafe { &*expr_global };
    return get_global_state(globals, object.name) == Global::Default
      && object.name == library
      && unsafe { (*expr).index == member };
  }

  false
}
