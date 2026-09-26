use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock},
  rtti::AstNodePtr,
  visit::ast_stat_visit,
};

use crate::records::find_node::FindNode;

impl FindNode {
  /// # Safety
  /// 调用方须保证 `node` 非空、对齐，指向 parse arena 中整个查询期存活且地址稳定的 `AstNode`；本函数
  /// 只读其 `location` 并把该裸指针存入 `self.best`，故 `node` 的存活期须覆盖到 `best` 被消费为止。
  /// cpp `Analysis/src/AstQuery.cpp:135`（`FindNode::visit(AstNode*)`）。单线程。
  pub unsafe fn visit_ast_node(&mut self, node: *mut AstNode) -> bool {
    // Safety: `node` 由本文件 visit_ast_stat_block 及 AstVisitor 遍历传入，指向 parse
    // arena 中整个查询期存活的节点（非空、对齐），此处只读 location 并把同址指针存入
    // best（与 C++ AstQuery::findNode 的裸指针语义一致）。
    let node_ref = unsafe { &*node };

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
    // Safety: block 来自 AstVisitor 对存活模块根节点的遍历，非空；AstStatBlock 为
    // repr(C) 且首字段链 base: AstNode 基址重合，上转满足 visit_ast_node 的存活
    // 节点入参契约。
    unsafe { self.visit_ast_node(block.as_ast_node()) };

    // Safety: 同上，block 指向 parse arena 存活节点，此处只读其 body 字段头。
    let block_ref = unsafe { &*block };
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
