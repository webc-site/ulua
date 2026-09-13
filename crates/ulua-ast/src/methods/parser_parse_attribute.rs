//! Node: `cxx:Method:Luau.Ast:Ast/src/Parser.cpp:994:parseAttribute`
//!
//! Faithful port of `Parser::parseAttribute` — parse one `@name` attribute, or
//! a bracketed `@[ name(args), ... ]` list. Attribute arguments must be literal
//! constants/tables; the name is validated against the known-attribute table.

use core::ptr::null_mut;

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{is_constant_literal::is_constant_literal, is_literal_table::is_literal_table},
  records::{
    ast_array::AstArray,
    ast_attr::{AstAttr, AstAttrType},
    ast_expr::AstExpr,
    lexeme::Type,
    location::Location,
    match_lexeme::MatchLexeme,
    parser::Parser,
    temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_attribute(&mut self, attributes: &mut TempVector<'_, *mut AstAttr>) {
    let empty: AstArray<*mut AstExpr> = AstArray {
      data: null_mut(),
      size: 0,
    };

    LUAU_ASSERT!(
      self.lexer.current().r#type == Type::ATTRIBUTE
        || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
    );

    if self.lexer.current().r#type == Type::ATTRIBUTE {
      let loc = self.lexer.current().location;
      let name = self.lexer.current().name();
      let ty = self.validate_attribute(loc, name.as_str_or_empty(), attributes, &empty);

      self.next_lexeme();

      let node = unsafe {
        (*self.allocator).alloc(
          AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
            loc,
            ty.unwrap_or(AstAttrType::Unknown),
            empty,
            name,
          ),
        )
      };
      attributes.push_back(node);
    } else {
      let open = *self.lexer.current();
      self.next_lexeme();

      if self.lexer.current().r#type != Type(b']' as i32) {
        loop {
          let name = self.parse_name("attribute name");
          let name_loc = name.location;
          let attr_name = name.name;

          let ct = self.lexer.current().r#type;
          if ct == Type::RAW_STRING
            || ct == Type::QUOTED_STRING
            || ct == Type(b'{' as i32)
            || ct == Type(b'(' as i32)
          {
            let (args, args_location, _expr_location) = self.parse_call_list(null_mut());

            for &arg in args.iter() {
              if !is_constant_literal(arg) && !is_literal_table(arg) {
                self.report(
                  args_location,
                  format_args!("Only literals can be passed as arguments for attributes"),
                );
              }
            }

            let ty =
              self.validate_attribute(name_loc, attr_name.as_str_or_empty(), attributes, &args);

            let node = unsafe {
              (*self.allocator).alloc(
                AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
                  Location::new(name_loc.begin, args_location.end),
                  ty.unwrap_or(AstAttrType::Unknown),
                  args,
                  attr_name,
                ),
              )
            };
            attributes.push_back(node);
          } else {
            let ty =
              self.validate_attribute(name_loc, attr_name.as_str_or_empty(), attributes, &empty);
            let node = unsafe {
              (*self.allocator).alloc(
                AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
                  name_loc,
                  ty.unwrap_or(AstAttrType::Unknown),
                  empty,
                  attr_name,
                ),
              )
            };
            attributes.push_back(node);
          }

          if self.lexer.current().r#type == Type(b',' as i32) {
            self.next_lexeme();
          } else {
            break;
          }
        }
      } else {
        let end_loc = self.lexer.current().location;
        self.report(
          Location::new(open.location.begin, end_loc.end),
          format_args!("Attribute list cannot be empty"),
        );

        // autocomplete expects at least one unknown attribute.
        let node = unsafe {
          (*self.allocator).alloc(
            AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
              Location::new(open.location.begin, end_loc.end),
              AstAttrType::Unknown,
              empty,
              self.name_error,
            ),
          )
        };
        attributes.push_back(node);
      }

      self.expect_match_and_consume(']', &MatchLexeme::new(&open), false);
    }
  }
}
