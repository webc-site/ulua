use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn get_identifier_of_base_var(node: *mut AstExpr) -> Option<String> {
  unsafe {
    let global = ast_node_as::<AstExprGlobal>(node as *mut AstNode);
    if !global.is_null() {
      let name_ptr = (*global).name.value;
      return if name_ptr.is_null() {
        Some(String::new())
      } else {
        Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
      };
    }

    let local = ast_node_as::<AstExprLocal>(node as *mut AstNode);
    if !local.is_null() {
      let name_ptr = (*(*local).local).name.value;
      return if name_ptr.is_null() {
        Some(String::new())
      } else {
        Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
      };
    }

    let index_expr = ast_node_as::<AstExprIndexExpr>(node as *mut AstNode);
    if !index_expr.is_null() {
      return get_identifier_of_base_var((*index_expr).expr);
    }

    let index_name = ast_node_as::<AstExprIndexName>(node as *mut AstNode);
    if !index_name.is_null() {
      return get_identifier_of_base_var((*index_name).expr);
    }

    None
  }
}
