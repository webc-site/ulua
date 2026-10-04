use core::ptr::{from_mut, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_function::AstStatFunction, ast_visitor::AstVisitor, position::Position,
  },
  visit,
};
#[derive(Debug, Clone)]
pub struct FindNode {
  pub(crate) pos: Position,
  pub(crate) document_end: Position,
  /// §2(b)：cpp `FindNode { AstNode* best; }`——AST 查询命中的节点句柄。其唯一
  /// 消费点 `find_node_at_position_*` 以 `*mut AstNode` 为**公开返回契约**，下游
  /// `find_expr_at_position → find_type_at_position` 再把节点指针用作
  /// `Module.ast_types: DenseHashMap<*const AstExpr, ..>` 的**裸指针键**做类型解析
  /// （见 functions/find_type_at_position.rs）。该 AST 查询链整条以指针身份运作，
  /// 单把本字段改 Option 会在返回/映射键边界逼出 `.as_ptr()` 倒灌或重写整条查询 API
  /// 与其跨 crate（ulua-unit-test）调用点，收益仅字段形态，故连同该子系统整体保留
  /// （同 records/nearest_statement_finder.rs、module.rs `root` 的一组 §2(b) 理由）。
  pub(crate) best: *mut AstNode,
}

impl FindNode {
  pub fn new(pos: Position, document_end: Position) -> Self {
    Self {
      pos,
      document_end,
      // 既有约定（review.md §2）：best 空 = 查询未命中，指针身份面见上方字段文档。
      best: null_mut(),
    }
  }
}

impl AstVisitor for FindNode {
  fn visit_node(&mut self, node: &mut AstNode) -> bool {
    if node.location.contains(self.pos) {
      self.best = from_mut(node);
      return true;
    }

    if node.location.end == self.document_end && self.pos >= self.document_end {
      self.best = from_mut(node);
      return true;
    }

    false
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_node(&mut node.base.base);
    // name/func 已句柄化为 Node（非空+存活由句柄契约承载），基类视图取
    // `.get()`、写穿分发落 ast_expr_visit_ref 引用形态，不再有裸指针解引用。
    let name_ref = node.name.get();
    let func_ref = node.func.get();
    if name_ref.base.location.contains(self.pos) {
      visit::ast_expr_visit_ref(node.name.get_mut(), self);
    } else if func_ref.base.base.location.contains(self.pos) {
      visit::ast_expr_visit_ref(node.func.cast::<AstExpr>().get_mut(), self);
    }
    false
  }

  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    self.visit_node(&mut node.base.base);
    for stat in node.body.iter_nodes_mut() {
      let stat_ref = stat.get();
      if stat_ref.base.location.end < self.pos {
        continue;
      }
      if stat_ref.base.location.begin > self.pos {
        break;
      }
      // 句柄 `get_mut()` 出借独占借用喂 `_ref` 门面，全链路 safe。
      visit::ast_stat_visit_ref(stat.get_mut(), self);
    }
    false
  }
}
