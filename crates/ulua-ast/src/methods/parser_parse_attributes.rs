use crate::records::{
  ast_array::AstArray, ast_attr::AstAttr, lexeme::Type, parser::Parser, temp_vector::TempVector,
};

impl Parser {
  // attributes ::= {attribute}
  pub fn parse_attributes(&mut self) -> AstArray<*mut AstAttr> {
    let r#type = self.lexer.current().r#type;

    ulua_common::macros::luau_assert::LUAU_ASSERT!(
      r#type == Type::ATTRIBUTE || r#type == Type::ATTRIBUTE_OPEN
    );

    let mut attributes = TempVector::new(&mut self.scratch_attr);

    while self.lexer.current().r#type == Type::ATTRIBUTE
      || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
    {
      self.parse_attribute(&mut attributes);
    }

    self.copy_temp_vector_t(&attributes)
  }
}
