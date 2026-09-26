use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_error::AstExprError, ast_node::AstNode,
    position::Position,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
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
  // Safety: expr 于上方判非空，指向 module AST arena 中存活的只读 AstExpr（repr(C)
  // 基类字段与派生类型基址重合）；try_as_ptr 按 class_index 甄别，未命中返回 None
  // 且从不解引用，命中即借出存活 AstExprCall 的只读引用，arena 地址不移动、于
  // `module` 借用期内存活。args.data/func 等子指针由 parser 按 data/size 成对写入；
  // 全程单线程只读，无别名冲突。
  if let Some(expr_call) = unsafe { ast_node_try_as_ptr::<AstExprCall>(expr.cast_mut()) } {
    let arg_location = expr_call.arg_location;

    if (expr_call.args.is_empty() && arg_location.contains(position))
      || expr_call
        .args
        .first()
        .is_some_and(|&first_arg| unsafe { ast_node_is_ptr::<AstExprError>(first_arg) })
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
