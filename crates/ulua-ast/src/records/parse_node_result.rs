use alloc::vec::Vec;

use crate::{
  records::{
    ast_node::AstNode, comment::Comment, hot_comment::HotComment, parse_error::ParseError,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

#[derive(Debug, Clone)]
pub struct ParseNodeResult<Node = AstNode> {
  pub root: *mut Node,
  pub lines: usize,
  pub hotcomments: Vec<HotComment>,
  pub errors: Vec<ParseError>,
  pub comment_locations: Vec<Comment>,
  pub cst_node_map: CstNodeMap,
}
