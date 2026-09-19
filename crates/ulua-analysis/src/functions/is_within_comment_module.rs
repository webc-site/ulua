use ulua_ast::records::{comment::Comment, lexeme::Type, position::Position};

use crate::functions::contains::contains;

pub fn is_within_comment(comment_locations: &[Comment], pos: Position) -> bool {
  let idx = comment_locations.partition_point(|c| {
    if c.r#type == Type::COMMENT {
      c.location.end.line < pos.line
    } else {
      c.location.end < pos
    }
  });

  if idx < comment_locations.len() {
    if contains(pos, comment_locations[idx]) {
      return true;
    }

    let next_idx = idx + 1;
    if next_idx < comment_locations.len() && contains(pos, comment_locations[next_idx]) {
      return true;
    }
  }

  false
}

pub use is_within_comment as is_within_comment_vector_comment_position;
