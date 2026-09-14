use alloc::{boxed::Box, string::String, vec::Vec};

use ulua_ast::records::{
  allocator::Allocator, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
  comment::Comment, position::Position,
};
#[derive(Debug)]
pub struct FragmentParseResult {
  pub fragment_to_parse: String,
  pub root: *mut AstStatBlock,
  pub ancestry: Vec<*mut AstNode>,
  pub nearest_statement: *mut AstStat,
  pub comment_locations: Vec<Comment>,
  pub alloc: Box<Allocator>,
  pub scope_pos: Position,
}
