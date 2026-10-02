use ulua_ast::records::{
  ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_visitor::AstVisitor, position::Position,
};

use crate::records::arena_handle::Handle;

#[derive(Debug, Clone)]
pub struct NearestStatementFinder {
  pub(crate) cursor: Position,
  /// §2：cpp `NearestStatementFinder` 的 `AstStat* nearestStatement` /
  /// `AstStatBlock* parent` 空槽句柄化：`None` ≡ 遍历未命中（cpp 初始
  /// `nullptr`），命中为 visit 期间 arena 存活节点；消费点
  /// （`get_fragment_region*` → `FragmentRegion`）已同为 `Option<Handle>` 形态，
  /// 指针身份判等经 `Handle` 的地址 `Eq` 逐位等价。
  pub(crate) nearest: Option<Handle<AstStat>>,
  pub(crate) parent: Option<Handle<AstStatBlock>>,
}

impl NearestStatementFinder {
  pub fn new(cursor_position: Position) -> Self {
    Self {
      cursor: cursor_position,
      // 既有约定（review.md §2）：两 None = 遍历未命中（cpp null 哨兵）。
      nearest: None,
      parent: None,
    }
  }
}

impl AstVisitor for NearestStatementFinder {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    let block_location = node.base.base.location;
    if !block_location.contains(self.cursor) && block_location.end != self.cursor {
      return false;
    }
    self.parent = Some(Handle::from_mut(node));

    // 取 begin <= cursor 的最后一条语句：反向首中即原「正向覆写到最后」的
    // 同一节点（谓词逐字未动，无并列重排）。
    if let Some(last) = node
      .body
      .iter_nodes()
      .rev()
      .find(|v| v.get().base.location.begin <= self.cursor)
    {
      self.nearest = Some(Handle::from_ptr(last.as_ptr()));
    }

    true
  }
}
