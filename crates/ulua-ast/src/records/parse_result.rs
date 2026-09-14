use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::{
  records::{
    ast_stat_block::AstStatBlock, comment::Comment, hot_comment::HotComment,
    parse_error::ParseError,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

#[derive(Debug, Clone)]
pub struct ParseResult {
  pub root: *mut AstStatBlock,
  pub lines: usize,
  pub hotcomments: Vec<HotComment>,
  pub errors: Vec<ParseError>,
  pub comment_locations: Vec<Comment>,
  pub cst_node_map: CstNodeMap,
}

impl Default for ParseResult {
  fn default() -> Self {
    Self {
      root: null_mut(),
      lines: 0,
      hotcomments: Vec::new(),
      errors: Vec::new(),
      comment_locations: Vec::new(),
      cst_node_map: CstNodeMap::new(null_mut()),
    }
  }
}
