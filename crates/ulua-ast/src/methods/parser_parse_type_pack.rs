use core::ptr::NonNull;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  records::{
    ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic, cst_type_pack_generic::CstTypePackGeneric,
    location::Location, parser::Parser,
  },
};

impl Parser {
  /// cpp `Parser::parseTypePack`（`Parser.cpp:3473`）。
  ///
  /// 返回 `None` 只发生在 cpp `LUAU_ASSERT(!"parseTypePack can't be called if
  /// shouldParseTypePack() returned false")`（`Parser.cpp:3498`）之后：断言可被 host 的
  /// assert handler 接管并继续执行，此时 cpp `return nullptr`，调用方把 null 写进可空槽位。
  /// 故这里用 `Option` 逐位保留该形态，而不是凭空假设它永不发生。
  pub fn parse_type_pack(&mut self) -> Option<NonNull<AstTypePack>> {
    if self.lexer.current().r#type == Type::DOT3 {
      let start = self.lexer.current().location;
      self.next_lexeme();
      let vararg_ty = self.parse_type(false);
      node_opt(self.alloc_type_pack(AstTypePackVariadic::new(
        // Safety: `parse_type` 依 parser 契约返回 arena 中存活的非空 `*mut AstType`
        // （bump 地址不移动），此处仅读其 `#[repr(C)]` 首字段 `base.location` 作为 pack
        // 节点坐标，不构造引用、不产生 `&mut`，无别名冲突。
        Location::new(start.begin, unsafe { (*vararg_ty).base.location.end }),
        vararg_ty,
      )))
    } else if self.lexer.current().r#type == Type::NAME
      && self.lexer.lookahead().r#type == Type::DOT3
    {
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
      node_opt(node)
    } else {
      LUAU_ASSERT!(false);
      None
    }
  }
}
