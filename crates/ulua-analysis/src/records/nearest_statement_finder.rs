use core::ptr::{from_mut, null_mut};

use ulua_ast::records::{
  ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_visitor::AstVisitor, position::Position,
};
#[derive(Debug, Clone)]
pub struct NearestStatementFinder {
  pub(crate) cursor: Position,
  /// §2(b)：cpp `NearestStatementFinder` 的 `AstStat* nearestStatement` /
  /// `AstStatBlock* parent`——遍历器累积的 AST 树句柄，语义确为「未命中=null」，
  /// 但其**唯一消费点**（`get_fragment_region*`）把二者原样写入 `FragmentRegion`
  /// 的裸字段 `nearest_statement: *mut AstStat`/`parent_block: *mut AstStatBlock`，
  /// 并透传给 `unsafe fn get_fragment_location(*mut AstStat, ..)`。该分片区域 API
  /// 整条为裸指针形态（同 records/fragment_parse_result.rs、module.rs `root`、
  /// find_node.rs `best` 的一组保留理由）。单改本记录两字段会在这些边界逼出
  /// `.as_ptr()` 倒灌或被迫重写整条 FragmentRegion 消费链，越出本任务范围、收益仅
  /// 字段形态，故连同该子系统整体保留。
  pub(crate) nearest: *mut AstStat,
  pub(crate) parent: *mut AstStatBlock,
}

impl NearestStatementFinder {
  pub fn new(cursor_position: Position) -> Self {
    Self {
      cursor: cursor_position,
      // 既有约定（review.md §2）：两空 = 遍历未命中，指针身份面见上方字段 §2(b) 块注释
      // （FragmentRegion 消费链整体保留）。
      nearest: null_mut(),
      parent: null_mut(),
    }
  }
}

impl AstVisitor for NearestStatementFinder {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    let block_location = node.base.base.location;
    if !block_location.contains(self.cursor) && block_location.end != self.cursor {
      return false;
    }
    self.parent = from_mut(node);

    // 取 begin <= cursor 的最后一条语句：反向首中即原「正向覆写到最后」的
    // 同一节点（谓词逐字未动，无并列重排）。
    if let Some(last) = node
      .body
      .iter_nodes()
      .rev()
      .find(|v| v.get().base.location.begin <= self.cursor)
    {
      self.nearest = last.as_ptr();
    }

    true
  }
}
