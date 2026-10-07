use ulua_ast::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
  ast_stat_continue::AstStatContinue, ast_stat_if::AstStatIf, ast_stat_return::AstStatReturn,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::is_constant::{is_constant_false, is_constant_true},
  records::{constant::Constant, node::Node},
};

/// 仅 crate 内判定用；`node` 为分发遍历交付的 arena 存活语句句柄
/// （非空由 [`Node`] 类型端承载，cpp 的 null 折叠分支在本形态下不可达）。
pub(crate) fn always_terminates(
  constants: &DenseHashMap<Node<AstExpr>, Constant>,
  node: Node<AstStat>,
) -> bool {
  // 顺序块：首条必终止的语句决定整体（句柄判型+下转：命中即 class 判定类型正确）
  if let Some(block) = node.try_as::<AstStatBlock>() {
    return block
      .body
      .iter_nodes()
      .any(|item| always_terminates(constants, Node::from(*item)));
  }

  // 句柄判型（只读 class 位、不外传借用）。
  if node.is::<AstStatReturn>() || node.is::<AstStatBreak>() || node.is::<AstStatContinue>() {
    return true;
  }

  // 条件块：常量条件只看活分支，否则两支都须终止
  // thenbody/elsebody 已句柄化，非空/判空由 Node/OptNode 类型端兑现。
  if let Some(stat_if) = node.try_as::<AstStatIf>() {
    let condition = stat_if.condition;
    let thenbody = Node::from(stat_if.thenbody).cast::<AstStat>();
    let elsebody = stat_if
      .elsebody
      .to_option()
      .map(|n| Node::from(n).cast::<AstStat>());

    if is_constant_true(constants, condition.into()) {
      // thenbody 静态类型是 AstStatBlock，向上转基类传参
      return always_terminates(constants, thenbody);
    }

    let Some(elsebody) = elsebody else {
      return false;
    };

    if is_constant_false(constants, condition.into()) {
      return always_terminates(constants, elsebody);
    }

    return always_terminates(constants, thenbody) && always_terminates(constants, elsebody);
  }

  false
}
