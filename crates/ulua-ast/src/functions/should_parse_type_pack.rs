use crate::records::{lexeme::Type, lexer::Lexer};

pub(crate) fn should_parse_type_pack(lexer: &mut Lexer) -> bool {
  lexer.current().r#type == Type::DOT3
    || (lexer.current().r#type == Type::NAME && lexer.lookahead().r#type == Type::DOT3)
}
