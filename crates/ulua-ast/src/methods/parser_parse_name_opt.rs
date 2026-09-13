use crate::records::{ast_name::AstName, lexeme::Type, name::Name, parser::Parser};

impl Parser {
  pub fn parse_name_opt(&mut self, context: &str) -> Option<Name> {
    if self.lexer.current().r#type != Type::NAME {
      self.report_name_error(context);

      return None;
    }

    let current = self.lexer.current();
    let result = Name {
      name: AstName {
        value: unsafe { current.data.name },
      },
      location: current.location,
    };

    self.next_lexeme();

    Some(result)
  }
}
