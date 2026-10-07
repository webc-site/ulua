//! `find_node` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock},
  rtti::AstNodePtr,
  visit::ast_stat_visit_ref,
};

use crate::records::{
  arena_handle::{alias_opt, alias_opt_mut},
  find_node::FindNode,
};

impl FindNode {
  /// 对 AST 节点位置进行命中检测并记录 best 节点指针。
  /// cpp `Analysis/src/AstQuery.cpp:135`（`FindNode::visit(AstNode*)`）。
  pub(crate) fn visit_ast_node(&mut self, node: *mut AstNode) -> bool {
    let Some(node_ref) = alias_opt(node) else {
      return false;
    };

    if node_ref.location.contains(self.pos) {
      self.best = node;
      return true;
    }

    if node_ref.location.end == self.document_end && self.pos >= self.document_end {
      self.best = node;
      return true;
    }

    false
  }

  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
    self.visit_ast_node(block.as_ast_node());

    let Some(block_ref) = alias_opt_mut(block) else {
      return false;
    };
    let body = &mut block_ref.body;

    for stat in body.iter_nodes_mut() {
      let stat_ref = stat.get();

      if stat_ref.base.location.end < self.pos {
        continue;
      }
      if stat_ref.base.location.begin > self.pos {
        break;
      }

      // 句柄 `get_mut()` 逐级出借独占借用喂 `_ref` 门面，全链路 safe（对应
      // cpp 裸指针分发路径，遍历序与时机不变）。
      ast_stat_visit_ref(stat.get_mut(), self);
    }

    false
  }
}
