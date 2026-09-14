use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn get_function_name_as_string(expr: &AstExpr) -> Option<String> {
  let mut curr = expr as *const AstExpr;
  let mut s = String::new();

  unsafe {
    loop {
      let local = ast_node_as::<AstExprLocal>(curr as *mut AstNode);
      if !local.is_null() {
        let name_ptr = (*(*local).local).name.value;
        let mut name = if name_ptr.is_null() {
          String::new()
        } else {
          CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
        };
        name.push_str(&s);
        return Some(name);
      }

      let global = ast_node_as::<AstExprGlobal>(curr as *mut AstNode);
      if !global.is_null() {
        let name_ptr = (*global).name.value;
        let mut name = if name_ptr.is_null() {
          String::new()
        } else {
          CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
        };
        name.push_str(&s);
        return Some(name);
      }

      let indexname = ast_node_as::<AstExprIndexName>(curr as *mut AstNode);
      if !indexname.is_null() {
        curr = (*indexname).expr;

        let index_ptr = (*indexname).index.value;
        let index_str = if index_ptr.is_null() {
          String::new()
        } else {
          CStr::from_ptr(index_ptr).to_string_lossy().into_owned()
        };

        let mut new_s = String::new();
        new_s.push('.');
        new_s.push_str(&index_str);
        new_s.push_str(&s);
        s = new_s;

        continue;
      }

      let group = ast_node_as::<AstExprGroup>(curr as *mut AstNode);
      if !group.is_null() {
        curr = (*group).expr;
        continue;
      }

      return None;
    }
  }
}
