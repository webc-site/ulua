use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_error::AstExprError, ast_node::AstNode,
    node_handle::OptNode, position::Position,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

use crate::{
  functions::{
    first::first, flatten_type_pack::flatten_type_pack_id, follow_type, get_type,
    is_variadic_type_pack::is_variadic,
  },
  records::{function_type::FunctionType, module::Module},
  type_aliases::type_id::TypeId,
};

pub fn find_expected_type_at(
  module: &Module,
  node: &AstNode,
  position: Position,
) -> Option<TypeId> {
  let expr: *const AstExpr = node.as_expr_const()?;

  // Extra care for first function call argument location
  // When we don't have anything inside () yet, we also don't have an AST node to base our lookup
  // 基类引用经句柄门面 `OptNode::from_ptr` 借出，判型下转走生命周期正确的
  // [`ast_node_try_as`]：借用半径由本函数局部句柄供给，不再锻造假 'static；
  // `args` 槽位判型用安全 [`ast_node_is`]（null 折叠为 false），ast_types map
  // 键保持裸指针身份形态。
  let expr_node = OptNode::from_ptr(expr.cast_mut());
  if let Some(expr_call) = expr_node
    .get()
    .and_then(|e| ast_node_try_as::<AstExprCall>(e))
  {
    let arg_location = expr_call.arg_location;

    if (expr_call.args.is_empty() && arg_location.contains(position))
      || expr_call.args.first().is_some_and(|&first_arg| {
        OptNode::from_ptr(first_arg)
          .get()
          .is_some_and(ast_node_is::<AstExprError>)
      })
    {
      let it = module.ast_types.find(&(expr_call.func as *const AstExpr));
      // `it?` 即"未命中早退 None"，与 cpp `*it` 前提同位（原 it?; + unwrap 双写合一）。
      let follow_ty = follow_type::follow(*it?);
      let ftv = get_type::get::<FunctionType>(follow_ty)?;

      let (head, tail) = flatten_type_pack_id(ftv.arg_types);
      let index = if expr_call.self_ { 1 } else { 0 };

      if index < head.len() {
        return Some(head[index]);
      } else if index == head.len()
        && let Some(tail_tp) = tail
        && is_variadic(tail_tp)
      {
        // C++ `first(*tail)` 默认 ignoreHiddenVariadics = true
        return first(tail_tp, true);
      }

      return None;
    }
  }

  let it = module.ast_expected_types.find(&expr);
  Some(*it?)
}
