use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_table::AstExprTable, ast_node::AstNode,
  },
  rtti,
};

pub fn get_table_hint(expr: *mut AstExpr) -> *mut AstExprTable {
  if expr.is_null() {
    return null_mut();
  }

  let table = unsafe { rtti::ast_node_as::<AstExprTable>(expr as *mut AstNode) };
  if !table.is_null() {
    return table;
  }

  let call = unsafe { rtti::ast_node_as::<AstExprCall>(expr as *mut AstNode) };
  if !call.is_null() {
    let call_ref = unsafe { &*call };
    if !call_ref.self_ && call_ref.args.size == 2 {
      let func = unsafe { rtti::ast_node_as::<AstExprGlobal>(call_ref.func as *mut AstNode) };
      if !func.is_null() {
        let func_ref = unsafe { &*func };
        if func_ref.name.operator_eq_c_char(c"setmetatable") {
          let arg0 = unsafe { *call_ref.args.data.add(0) };
          let table_arg = unsafe { rtti::ast_node_as::<AstExprTable>(arg0 as *mut AstNode) };
          if !table_arg.is_null() {
            return table_arg;
          }
        }
      }
    }
  }

  null_mut()
}
