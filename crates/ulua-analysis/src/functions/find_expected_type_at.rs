use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_error::AstExprError, ast_node::AstNode,
    position::Position,
  },
  rtti::{ast_node_as, ast_node_is},
};

use crate::{
  functions::{
    first::first, flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, is_variadic_type_pack::is_variadic,
  },
  records::{function_type::FunctionType, module::Module},
  type_aliases::type_id::TypeId,
};

pub fn find_expected_type_at(
  module: &Module,
  node: &AstNode,
  position: Position,
) -> Option<TypeId> {
  let expr = node.as_expr_const() as *mut AstExpr;
  if expr.is_null() {
    return None;
  }

  // Extra care for first function call argument location
  // When we don't have anything inside () yet, we also don't have an AST node to base our lookup
  if unsafe { ast_node_is::<AstExprCall>(&(*expr).base) } {
    let expr_call = unsafe { ast_node_as::<AstExprCall>(expr as *mut AstNode) };
    if !expr_call.is_null() {
      let args_size = unsafe { (*expr_call).args.size };
      let arg_location = unsafe { (*expr_call).arg_location };

      if (args_size == 0 && arg_location.contains(position))
        || (args_size > 0
          && unsafe {
            let first_arg = *(*expr_call).args.data;
            ast_node_is::<AstExprError>(&(*first_arg).base)
          })
      {
        let it = module
          .ast_types
          .find(&(unsafe { (*expr_call).func } as *const AstExpr));
        it?;

        let follow_ty = follow_type_id(*it.unwrap());
        let ftv = get_type_id::<FunctionType>(follow_ty)?;

        let (head, tail) = flatten_type_pack_id(ftv.arg_types);
        let index = if unsafe { (*expr_call).self_ } { 1 } else { 0 };

        if index < head.len() {
          return Some(head[index as usize]);
        } else if index == head.len() && tail.is_some() {
          let tail_tp = tail.unwrap();
          if is_variadic(tail_tp) {
            return first(tail_tp, false);
          }
        }

        return None;
      }
    }
  }

  let it = module.ast_expected_types.find(&(expr as *const AstExpr));
  it?;

  Some(*it.unwrap())
}
