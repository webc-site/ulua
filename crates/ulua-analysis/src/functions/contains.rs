use ulua_ast::{
  enums::type_lexer::Type,
  records::{comment::Comment, position::Position},
};

pub fn contains(pos: Position, comment: Comment) -> bool {
  if comment.location.contains(pos)
    || (comment.r#type == Type::BROKEN_COMMENT && comment.location.begin <= pos)
  {
    true
  } else {
    comment.r#type == Type::COMMENT
      && comment.location.end.line == pos.line
      && comment.location.begin <= pos
  }
}
