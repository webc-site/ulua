use alloc::vec::Vec;
use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

pub fn parse_path_expr(path_expr: &AstExpr) -> Vec<&str> {
  let mut index_name =
    unsafe { ast_node_as::<AstExprIndexName>(path_expr as *const AstExpr as *mut AstNode) };

  if index_name.is_null() {
    return Vec::new();
  }

  let mut segments: Vec<&str> = Vec::new();

  unsafe {
    segments.push(
      CStr::from_ptr((*index_name).index.value)
        .to_str()
        .unwrap_or(""),
    );

    loop {
      let next_expr = (*index_name).expr;
      if next_expr.is_null() {
        break;
      }

      let next_node = next_expr as *mut AstNode;

      if let Some(in_node) = ast_node_as::<AstExprIndexName>(next_node).as_mut() {
        segments.push(CStr::from_ptr(in_node.index.value).to_str().unwrap_or(""));
        index_name = in_node;
        continue;
      } else if let Some(global) = ast_node_as::<AstExprGlobal>(next_node).as_ref() {
        segments.push(CStr::from_ptr(global.name.value).to_str().unwrap_or(""));
        break;
      } else if let Some(local_expr) = ast_node_as::<AstExprLocal>(next_node).as_ref() {
        if !local_expr.local.is_null() {
          segments.push(
            CStr::from_ptr((*local_expr.local).name.value)
              .to_str()
              .unwrap_or(""),
          );
        }
        break;
      } else {
        return Vec::new();
      }
    }
  }

  segments.reverse();
  segments
}
