use ulua_ast::{
  enums::type_lexer::Type,
  records::{comment::Comment, parse_result::ParseResult, position::Position},
};

use crate::{functions::contains::contains, records::source_module::SourceModule};

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

pub fn is_within_comment_source_module_position(
  source_module: &SourceModule,
  pos: Position,
) -> bool {
  is_within_comment_vector_comment_position(&source_module.comment_locations, pos)
}

pub fn is_within_comment_parse_result_position(result: &ParseResult, pos: Position) -> bool {
  is_within_comment(&result.comment_locations, pos)
}
