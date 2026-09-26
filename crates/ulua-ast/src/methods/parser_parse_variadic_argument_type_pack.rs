use crate::{
  enums::type_lexer::Type,
  records::{
    ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic, cst_type_pack_generic::CstTypePackGeneric,
    location::Location, parser::Parser,
  },
};

impl Parser {
  pub fn parse_variadic_argument_type_pack(&mut self) -> *mut AstTypePack {
    if self.lexer.current().r#type == Type::NAME && self.lexer.lookahead().r#type == Type::DOT3 {
      let name = self.parse_name("generic name");
      let end = self.lexer.current().location;
      self.expect_and_consume_type(Type::DOT3, "generic type pack annotation");
      let node = self.alloc_type_pack(AstTypePackGeneric::new(
        Location::new(name.location.begin, end.end),
        name.name,
      ));
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstTypePackGeneric::new(end.begin))
      });
      node
    } else {
      let variadic_annotation = self.parse_type(false);
      self.alloc_type_pack(AstTypePackVariadic::new(
        // Safety: `parse_type` 依 parser 契约返回 arena 中存活的非空 `*mut AstType`；
        // 此处仅读其 `#[repr(C)]` 首字段 `base.location`（Location 为 Copy），不构造引用、
        // 不产生 `&mut`，无别名冲突。
        unsafe { (*variadic_annotation).base.location },
        variadic_annotation,
      ))
    }
  }
}
