use core::ptr::NonNull;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::{opt_node, slot_ref},
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_type::AstType,
    ast_type_function::AstTypeFunction, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_reference::AstTypeReference, lexeme::Lexeme, location::Location, parser::Parser,
    position::Position,
  },
  type_aliases::ast_argument_name::AstArgumentName,
};

impl Parser {
  pub fn parse_function_type_tail(
    &mut self,
    begin: &Lexeme,
    attributes: AstArray<*mut AstAttr>,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
    params: AstArray<*mut AstType>,
    param_names: AstArray<Option<AstArgumentName>>,
    vararg_annotation: Option<NonNull<AstTypePack>>,
  ) -> *mut AstType {
    self.increment_recursion_counter("type annotation");

    if self.lexer.current().r#type == Type::COLON {
      self.report(
        self.lexer.current().location,
        format_args!(
          "Return types in function type annotations are written after '->' instead of ':'"
        ),
      );
      self.next_lexeme();
    } else if self.lexer.current().r#type != Type::SKINNY_ARROW
      && generics.size == 0
      && generic_packs.size == 0
      && params.size == 0
    {
      self.report(
        Location::new(begin.location.begin, self.lexer.previous_location().end),
        format_args!("Expected '->' after '()' when parsing function type; did you mean 'nil'?"),
      );
      return self.alloc_type(AstTypeReference::new(
        begin.location,
        None,
        self.name_nil,
        None,
        begin.location,
        false,
        AstArray::EMPTY,
      ));
    } else {
      self.expect_and_consume_type(Type::SKINNY_ARROW, "function type");
    }

    let return_type = self.parse_return_type();
    LUAU_ASSERT!(return_type.is_some());

    // cpp `LUAU_ASSERT(returnType)` 之后直接 `returnType->location`（`Parser.cpp:3106-3112`）：
    // null 形态在 cpp 里就是崩溃，且它已被上游 `shouldParseTypePack()` 守卫排除。这里取
    // `missing` 坐标兜住「host 接管断言后继续执行」的极端形态，尾槽仍按 cpp 写回同一指针值。
    let return_end = return_type.map_or(Position::missing(), |node| {
      slot_ref(node.as_ptr()).base.location.end
    });

    self.alloc_type(AstTypeFunction::ast_type_function_location_ast_array_ast_attr_ast_array_ast_generic_type_ast_array_ast_generic_type_pack_ast_type_list_ast_array_optional_ast_argument_name_ast_type_pack(
            Location::new(begin.location.begin, return_end),
            attributes,
            generics,
            generic_packs,
            AstTypeList::new(params, vararg_annotation),
            param_names,
            opt_node(return_type),
        ))
  }
}
