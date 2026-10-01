use ulua_ast::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
  ast_stat_continue::AstStatContinue, ast_stat_if::AstStatIf, ast_stat_return::AstStatReturn,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    ast_slot_ref::{ast_slot_is, ast_slot_try_as},
    is_constant::{is_constant_false, is_constant_true},
  },
  records::{constant::Constant, node::Node},
};

/// 仅 crate 内判定用；node 为 null 或指向存活节点（同 C++ 前置条件）。
pub(crate) fn always_terminates(
  constants: &DenseHashMap<Node<AstExpr>, Constant>,
  node: *mut AstStat,
) -> bool {
  // 顺序块：首条必终止的语句决定整体
  // 门面判型+下转：null 输入返回 None，命中即 class_index 判定类型正确
  // （node 契约为 cost 模型分发遍历交付的 arena 存活 AstStat）。
  if let Some(block) = ast_slot_try_as::<AstStatBlock, _>(node) {
    return block
      .body
      .iter_nodes()
      .any(|item| always_terminates(constants, item.as_ptr()));
  }

  // 门面判型（只读 class index、不外传借用，null 折叠 false）。
  if ast_slot_is::<AstStatReturn, _>(node)
    || ast_slot_is::<AstStatBreak, _>(node)
    || ast_slot_is::<AstStatContinue, _>(node)
  {
    return true;
  }

  // 条件块：常量条件只看活分支，否则两支都须终止
  // 门面判型+下转：thenbody/elsebody 已句柄化，非空/判空由 Node/OptNode 类型端
  // 兑现，as_ptr 仅作既有裸指针 API 的桥接。
  if let Some(stat_if) = ast_slot_try_as::<AstStatIf, _>(node) {
    let condition = stat_if.condition;
    let thenbody = stat_if.thenbody;
    let elsebody = stat_if.elsebody;

    if is_constant_true(constants, condition.into()) {
      // thenbody 静态类型是 AstStatBlock，向上转基类传参
      return always_terminates(constants, thenbody.as_ptr().cast::<AstStat>());
    }

    if is_constant_false(constants, condition.into()) && elsebody.is_some() {
      return always_terminates(constants, elsebody.as_ptr());
    }

    return elsebody.is_some()
      && always_terminates(constants, thenbody.as_ptr().cast::<AstStat>())
      && always_terminates(constants, elsebody.as_ptr());
  }

  false
}
