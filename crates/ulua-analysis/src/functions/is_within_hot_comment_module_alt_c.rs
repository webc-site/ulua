use ulua_ast::records::{parse_result::ParseResult, position::Position};

use crate::functions::is_within_hot_comment_module::is_within_hot_comment_vector_hot_comment_position;

pub fn is_within_hot_comment_parse_result_position(result: &ParseResult, pos: Position) -> bool {
  is_within_hot_comment_vector_hot_comment_position(&result.hotcomments, pos)
}
