use core::ptr::{from_ref, null};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

use crate::functions::{does_call_error::does_call_error, has_break::has_break};
pub fn get_fallthrough(node: *const AstStat) -> *const AstStat {
  if node.is_null() {
    return null();
  }

  // SAFETY: node 上方已判空非 null；AST 由解析器 arena 持有，本函数全程只读。
  // 仅此处一次裸解引用换得 &AstNode，后续下转与字段访问全部走安全引用。
  let node_ref: &AstNode = unsafe { &*(node as *const AstNode) };

  // if (auto block = node->as<AstStatBlock>())
  if let Some(block) = ast_node_try_as::<AstStatBlock>(node_ref) {
    if block.body.is_empty() {
      return from_ref(&block.base);
    }

    let body = &block.body;
    // 除最后一条外的所有语句都必须能"穿透"（不含 return/break）
    for s in &body.as_slice()[..body.len() - 1] {
      if get_fallthrough(s.as_ptr().cast_const()).is_null() {
        return null();
      }
    }

    return get_fallthrough(body[body.len() - 1].as_ptr().cast_const());
  }

  // else if (auto iff = node->as<AstStatIf>())
  if let Some(iff) = ast_node_try_as::<AstStatIf>(node_ref) {
    let thenf = get_fallthrough(iff.thenbody.cast::<AstStat>().as_ptr().cast_const());
    if !thenf.is_null() {
      return thenf;
    }

    if let Some(else_stat) = iff.elsebody.get() {
      let elsef = get_fallthrough(from_ref(else_stat));
      if !elsef.is_null() {
        return elsef;
      }
      return null();
    } else {
      return node;
    }
  }

  if ast_node_is::<AstStatReturn>(node_ref) {
    return null();
  }

  // else if (auto expr = node->as<AstStatExpr>())
  if let Some(stat_expr) = ast_node_try_as::<AstStatExpr>(node_ref) {
    // expr 已句柄化为非空 Node：cpp 的 `expr != nullptr` 守卫随类型消失，
    // 判型走安全的引用门面，原 `unsafe` 裸指针下转点不再需要。
    if let Some(call) = ast_node_try_as::<AstExprCall>(stat_expr.expr.get())
      && does_call_error(call)
    {
      return null();
    }

    return node;
  }

  // else if (auto while_ = node->as<AstStatWhile>())
  if let Some(while_) = ast_node_try_as::<AstStatWhile>(node_ref) {
    if let Some(cond) = ast_node_try_as::<AstExprConstantBool>(while_.condition.get())
      && cond.value
      && !has_break(while_.body.as_ptr().cast::<AstStat>())
    {
      return null();
    }

    return node;
  }

  // else if (auto repeat = node->as<AstStatRepeat>())
  if let Some(repeat) = ast_node_try_as::<AstStatRepeat>(node_ref) {
    // condition/body 已句柄化为 Node（非空由类型层承载）：原 `is_null` 防御
    // 守卫随类型消失，判型走安全引用门面（同上方 while 分支）。
    if let Some(cond) = ast_node_try_as::<AstExprConstantBool>(repeat.condition.get())
      && !cond.value
      && !has_break(repeat.body.as_ptr().cast::<AstStat>())
    {
      return null();
    }

    if get_fallthrough(repeat.body.as_ptr().cast::<AstStat>()).is_null() {
      return null();
    }

    return node;
  }

  node
}
