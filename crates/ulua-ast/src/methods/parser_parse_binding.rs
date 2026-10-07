use crate::{
  enums::type_lexer::Type,
  records::{binding::Binding, name::Name, parser::Parser, position::Position},
};

impl Parser {
  pub fn parse_binding(&mut self, is_const: bool) -> Binding {
    let name = self.parse_name_opt("variable name");

    let name = name.unwrap_or_else(|| Name {
      name: self.name_error,
      location: self.lexer.current().location,
    });

    let colon_position = if self.lexer.current().r#type == Type::COLON {
      self.lexer.current().location.begin
    } else {
      Position::missing()
    };
    let annotation = self.parse_optional_type();

    if self.options.store_cst_data {
      Binding::new(name, annotation, colon_position, is_const)
    } else {
      Binding::new(name, annotation, Position::missing(), is_const)
    }
  }
}
