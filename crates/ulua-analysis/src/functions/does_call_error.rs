use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_global::AstExprGlobal, node_handle::OptNode,
  },
  rtti::ast_node_try_as,
};
pub fn does_call_error(call: &AstExprCall) -> bool {
  // `func`/`args` 仍是 records 引用化波次前的裸指针字段：先经句柄门面
  // `OptNode::from_ptr` 把可空性收进 Option，判型下转走生命周期正确的
  // `ast_node_try_as`，借用半径由本函数内的局部句柄供给，不再锻造 'static。
  let func = OptNode::from_ptr(call.func);
  let Some(global) = func.get().and_then(|f| ast_node_try_as::<AstExprGlobal>(f)) else {
    return false;
  };

  // AstName::as_bytes 容忍 null（空名 → 空切片，两个分支均不命中）。
  let name_bytes = global.name.as_bytes();
  if name_bytes == b"error" {
    return true;
  }

  if name_bytes == b"assert" {
    // assert() will error because it is missing the first argument
    let Some(first_arg_ptr) = call.args.iter().next().copied() else {
      return true;
    };

    let first_arg = OptNode::from_ptr(first_arg_ptr);

    let Some(expr) = first_arg
      .get()
      .and_then(|a| ast_node_try_as::<AstExprConstantBool>(a))
    else {
      return false;
    };

    if !expr.value {
      return true;
    }
  }

  false
}
