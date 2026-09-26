use crate::{
  enums::type_lexer::Type,
  records::{lexeme::Lexeme, parser::Parser},
};

impl Parser {
  pub(crate) fn block_follow(&self, l: &Lexeme) -> bool {
    l.r#type == Type::EOF
      || l.r#type == Type::RESERVED_ELSE
      || l.r#type == Type::RESERVED_ELSEIF
      || l.r#type == Type::RESERVED_END
      || l.r#type == Type::RESERVED_UNTIL
  }
}
