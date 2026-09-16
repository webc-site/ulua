use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_global::AstExprGlobal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn does_call_error(call: &AstExprCall) -> bool {
  let global = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };

  if global.is_null() {
    return false;
  }

  unsafe {
    // AstName::as_bytes 容忍 null（空名 → 空切片，两个分支均不命中）。
    let name_bytes = (*global).name.as_bytes();
    if name_bytes == b"error" {
      return true;
    }

    if name_bytes == b"assert" {
      // assert() will error because it is missing the first argument
      let first_arg = match call.args.iter().next() {
        Some(arg) => *arg,
        None => return true,
      };

      if first_arg.is_null() {
        return false;
      }

      let expr = ast_node_as::<AstExprConstantBool>(first_arg as *mut AstNode);

      if expr.is_null() {
        return false;
      }

      if !(*expr).value {
        return true;
      }
    }
  }

  false
}
