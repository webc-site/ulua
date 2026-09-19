use crate::{
  records::{
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_block::AstStatBlock,
    printer::{IntoNodePtr, Printer},
    writer::Writer,
  },
  rtti::ast_node_try_as,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_block_ast_stat<S: IntoNodePtr<AstStat>>(&mut self, stat: S) {
    // SAFETY: stat 指向 arena 中存活的 AstStat 派生节点；class_index 匹配后
    // #[repr(C)] 单继承布局保证下转有效（与 ast_node_try_as 同一收口）。
    let stat = unsafe { &*stat.into_node_ptr() };
    if let Some(block_ref) = ast_node_try_as::<AstStatBlock>(&stat.base) {
      self.visualize_block_ast_stat_block(block_ref);
      return;
    }

    ulua_common::LUAU_ASSERT!(false);
  }
}
