use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position};

use crate::records::find_node::FindNode;

pub fn find_node_at_position_ast_stat_block_position(
  root: &AstStatBlock,
  mut pos: Position,
) -> *mut AstNode {
  let root_node = root as *const AstStatBlock as *const AstNode;
  let root_location = unsafe { (*root_node).location };
  let end = root_location.end;

  if pos < root_location.begin {
    return root_node as *mut AstNode;
  }

  if pos > end {
    pos = end;
  }

  let mut find_node = FindNode::new(pos, end);
  find_node.visit_ast_stat_block(root as *const AstStatBlock as *mut AstStatBlock);
  find_node.best
}
