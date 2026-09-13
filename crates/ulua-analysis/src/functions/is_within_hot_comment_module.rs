use alloc::vec::Vec;

use ulua_ast::records::{hot_comment::HotComment, position::Position};
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
