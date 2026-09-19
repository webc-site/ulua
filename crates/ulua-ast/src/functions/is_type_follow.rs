use crate::records::lexeme::Type;

pub(crate) fn is_type_follow(c: Type) -> bool {
  c == Type('|' as i32) || c == Type('?' as i32) || c == Type('&' as i32)
}
