use alloc::vec::Vec;

use ulua_ast::records::{hot_comment::HotComment, parse_result::ParseResult, position::Position};

use crate::records::source_module::SourceModule;

pub fn is_within_hot_comment_vector_hot_comment_position(
  hot_comments: &Vec<HotComment>,
  pos: Position,
) -> bool {
  for hot_comment in hot_comments {
    // The C++ source calls hotComment.location.contains_closed(pos).
    // In luau-ast, Location methods are inherent impls.
    // The previous attempt failed because it used snake_case 'contains_closed',
    // but the dependency card shows the Rust method name is 'containsClosed'.
    if hot_comment.location.contains_closed(pos) {
      return true;
    }
  }

  false
}

pub fn is_within_hot_comment_source_module_position(
  source_module: &SourceModule,
  pos: Position,
) -> bool {
  is_within_hot_comment_vector_hot_comment_position(&source_module.hotcomments, pos)
}

pub fn is_within_hot_comment_parse_result_position(result: &ParseResult, pos: Position) -> bool {
  is_within_hot_comment_vector_hot_comment_position(&result.hotcomments, pos)
}
