use core::ffi::c_char;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::{ast_node_as, ast_node_is},
};

use crate::{
  enums::parentheses_recommendation::ParenthesesRecommendation,
  functions::{flatten_type_pack::flatten_type_pack_id, is_variadic_type_pack::is_variadic},
  records::function_type::FunctionType,
};
pub fn get_paren_recommendation_for_func(
  func: &FunctionType,
  nodes: &[*mut AstNode],
) -> ParenthesesRecommendation {
  if already_has_parens_vec(nodes) {
    return ParenthesesRecommendation::None;
  }

  let last_node = *nodes.last().unwrap();
  let idx_expr = unsafe { ast_node_as::<AstExprIndexName>(last_node) };
  let has_implicit_self = !idx_expr.is_null() && unsafe { (*idx_expr).op == ':' as c_char };

  let (arg_types, arg_variadic_pack) = flatten_type_pack_id(func.arg_types);

  if let Some(variadic_pack) = arg_variadic_pack
    && is_variadic(variadic_pack)
  {
    return ParenthesesRecommendation::CursorInside;
  }

  let no_arg_function = arg_types.is_empty() || (has_implicit_self && arg_types.len() == 1);
  if no_arg_function {
    ParenthesesRecommendation::CursorAfter
  } else {
    ParenthesesRecommendation::CursorInside
  }
}

fn already_has_parens_vec(nodes: &[*mut AstNode]) -> bool {
  if nodes.is_empty() {
    return false;
  }

  let mut iter = nodes.len();
  while iter > 0 {
    iter -= 1;
    let node = nodes[iter];
    let is_valid = unsafe {
      ast_node_is::<AstExprLocal>(&*node)
        || ast_node_is::<AstExprGlobal>(&*node)
        || ast_node_is::<AstExprIndexName>(&*node)
        || ast_node_is::<AstExprIndexExpr>(&*node)
    };
    if !is_valid {
      break;
    }
  }

  if iter == nodes.len() || iter == 0 {
    return false;
  }

  let current_node = nodes[iter];
  let call = unsafe { ast_node_as::<AstExprCall>(current_node) };
  if call.is_null() {
    return false;
  }

  let prev_node = nodes[iter - 1];
  unsafe { (*call).func == prev_node as *mut AstExpr }
}
