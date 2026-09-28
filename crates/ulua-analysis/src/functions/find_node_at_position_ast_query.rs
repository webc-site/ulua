use core::ptr::{from_ref, null_mut};

use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position};

use crate::records::{find_node::FindNode, source_module::SourceModule};

pub fn find_node_at_position_source_module_position(
  source: &SourceModule,
  pos: Position,
) -> *mut AstNode {
  if source.root.is_null() {
    return null_mut();
  }
  // Safety: 上一行已判空 ⇒ source.root 非空；它指向 parse 树根 AstStatBlock，
  // 由 SourceModule.allocator（Arc<Allocator>，随 &source 存活）的 bump 堆块
  // 分配，块地址不移动，节点在本查询期间存活；重建只读 & 与并发无冲突。
  find_node_at_position_ast_stat_block_position(unsafe { &*source.root }, pos)
}

pub fn find_node_at_position_ast_stat_block_position(
  root: &AstStatBlock,
  mut pos: Position,
) -> *mut AstNode {
  let root_node = from_ref(root).cast::<AstNode>();
  // 安全替代原 `(*root_node).location`：AstStatBlock/AstStat 均为 repr(C) 且
  // 首字段链 base→base→AstNode.location，经类型路径直读同一 location，无 unsafe。
  let root_location = root.base.base.location;
  let end = root_location.end;

  if pos < root_location.begin {
    return root_node.cast_mut();
  }

  if pos > end {
    pos = end;
  }

  let mut find_node = FindNode::new(pos, end);
  find_node.visit_ast_stat_block(from_ref(root).cast_mut());
  find_node.best
}
