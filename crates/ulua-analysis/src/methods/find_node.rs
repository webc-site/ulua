//! `find_node` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock},
  rtti::AstNodePtr,
  visit::ast_stat_visit,
};

use crate::records::find_node::FindNode;

#[cfg(any())]
impl FindNode {
  pub fn new(pos: Position, document_end: Position) -> Self {
    Self {
      pos,
      document_end,
      best: core::ptr::null_mut(),
    }
  }
}

impl FindNode {
  /// 对 AST 节点位置进行命中检测并记录 best 节点指针。
  /// cpp `Analysis/src/AstQuery.cpp:135`（`FindNode::visit(AstNode*)`）。
  pub(crate) fn visit_ast_node(&mut self, node: *mut AstNode) -> bool {
    let Some(node_ref) = (unsafe { node.as_ref() }) else {
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

    let Some(block_ref) = (unsafe { block.as_ref() }) else {
      return false;
    };
    let body = &block_ref.body;

    for stat in body.iter_nodes() {
      let stat_ref = stat.get();

      if stat_ref.base.location.end < self.pos {
        continue;
      }
      if stat_ref.base.location.begin > self.pos {
        break;
      }

      // Safety: stat 同上为非空存活 AstStat 指针，self 实现 AstVisitor；与 C++
      // stat->visit(visitor) 的动态分派同前提。
      unsafe {
        ast_stat_visit(stat.as_ptr(), self);
      }
    }

    false
  }
}
