use crate::records::{
  ast_node::AstNode, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
  ast_type_pack_variadic::AstTypePackVariadic, cst_node::CstNode,
  cst_type_pack_generic::CstTypePackGeneric, lexeme::Type, location::Location, parser::Parser,
};

impl Parser {
  pub fn parse_variadic_argument_type_pack(&mut self) -> *mut AstTypePack {
    if self.lexer.current().r#type == Type::NAME && self.lexer.lookahead().r#type == Type::DOT3 {
      let name = self.parse_name("generic name");
      let end = self.lexer.current().location;
      self.expect_and_consume_type(Type::DOT3, "generic type pack annotation");
      let node = unsafe {
        (*self.allocator).alloc(AstTypePackGeneric::new(
          Location::new(name.location.begin, end.end),
          name.name,
        ))
      };
      self.attach_cst(node, |alloc| alloc.alloc(CstTypePackGeneric::new(end.begin)));
      node as *mut AstTypePack
    } else {
      let variadic_annotation = self.parse_type(false);
      unsafe {
        (*self.allocator).alloc(AstTypePackVariadic::new(
          (*variadic_annotation).base.location,
          variadic_annotation,
        )) as *mut AstTypePack
      }
    }
  }
}
