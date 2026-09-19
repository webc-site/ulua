use ulua_ast::{enums::type_lexer::Type, records::lexer::Lexer};

pub(crate) fn next(lexer: &mut Lexer) {
  lexer.next_lexeme();

  // skip C-style comments as Lexer only understands Lua-style comments atm
  while lexer.current().r#type == Type::FLOOR_DIV {
    lexer.nextline();
  }
}
