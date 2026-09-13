use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile,
  },
  rtti::{ast_node_as, ast_node_is},
};

use crate::functions::{does_call_error::does_call_error, has_break::has_break};
pub fn get_fallthrough(node: *const AstStat) -> *const AstStat {
  if node.is_null() {
    return null();
  }

  unsafe {
    if !ast_node_as::<AstStatBlock>(node as *const _ as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatBlock>(node as *const _ as *mut AstNode);
      if stat.is_null() {
        return null();
      }

      if (*stat).body.size == 0 {
        return stat as *const AstStat;
      }

      let size = (*stat).body.size;
      for i in 0..(size.saturating_sub(1)) {
        let s = *(*stat).body.data.add(i);
        if get_fallthrough(s).is_null() {
          return null();
        }
      }

      let last = *(*stat).body.data.add(size - 1);
      return get_fallthrough(last);
    }

    if !ast_node_as::<AstStatIf>(node as *const _ as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatIf>(node as *const _ as *mut AstNode);
      if stat.is_null() {
        return null();
      }

      let thenf = get_fallthrough((*stat).thenbody as *const AstStat);
      if !thenf.is_null() {
        return thenf;
      }

      if !(*stat).elsebody.is_null() {
        let elsef = get_fallthrough((*stat).elsebody);
        if !elsef.is_null() {
          return elsef;
        }
        return null();
      } else {
        return node;
      }
    }

    if ast_node_is::<AstStatReturn>(&*(node as *const AstNode)) {
      return null();
    }

    if !ast_node_as::<AstStatExpr>(node as *const _ as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatExpr>(node as *const _ as *mut AstNode);
      if stat.is_null() {
        return null();
      }

      if !(*stat).expr.is_null() {
        let call = ast_node_as::<AstExprCall>((*stat).expr as *mut AstNode);
        if !call.is_null() && does_call_error(&*call) {
          return null();
        }
      }

      return node;
    }

    if !ast_node_as::<AstStatWhile>(node as *const _ as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatWhile>(node as *const _ as *mut AstNode);
      if stat.is_null() {
        return null();
      }

      if !(*stat).condition.is_null() {
        let expr = ast_node_as::<AstExprConstantBool>((*stat).condition as *mut AstNode);
        if !expr.is_null() && (*expr).value && !has_break((*stat).body as *mut AstStat) {
          return null();
        }
      }

      return node;
    }

    if !ast_node_as::<AstStatRepeat>(node as *const _ as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatRepeat>(node as *const _ as *mut AstNode);
      if stat.is_null() {
        return null();
      }

      if !(*stat).condition.is_null() {
        let expr = ast_node_as::<AstExprConstantBool>((*stat).condition as *mut AstNode);
        if !expr.is_null() && !(*expr).value && !has_break((*stat).body as *mut AstStat) {
          return null();
        }
      }

      if get_fallthrough((*stat).body as *const AstStat).is_null() {
        return null();
      }

      return node;
    }

    node
  }
}
