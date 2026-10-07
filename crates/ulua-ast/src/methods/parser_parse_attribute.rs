//! Source: `Ast/src/Parser.cpp:994`
//!
//! Faithful port of `Parser::parseAttribute` — parse one `@name` attribute, or
//! a bracketed `@[ name(args), ... ]` list. Attribute arguments must be literal
//! constants/tables; the name is validated against the known-attribute table.

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::{is_constant_literal::is_constant_literal, is_literal_table::is_literal_table},
  records::{
    ast_attr::{AstAttr, AstAttrType},
    location::Location,
    match_lexeme::MatchLexeme,
    node_handle::{Node, Nodes},
    parser::Parser,
    temp_vector::TempVector,
  },
};

impl Parser {
  /// cpp `Parser::parseAttribute(TempVector<AstAttr*>& attributes)`：`attributes`
  /// 是调用方 `scratch_attr` 的窗口，元素以 [`Node`] 入栈（`alloc` 恒非空）。
  pub fn parse_attribute(&mut self, attributes: &mut TempVector<'_, Node<AstAttr>>) {
    LUAU_ASSERT!(
      self.lexer.current().r#type == Type::ATTRIBUTE
        || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
    );

    if self.lexer.current().r#type == Type::ATTRIBUTE {
      let loc = self.lexer.current().location;
      let name = self.lexer.current().name();
      let ty = self.validate_attribute(loc, name.as_str_or_empty(), attributes, &Nodes::empty());

      self.next_lexeme();

      let node = self.alloc(
        AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
          loc,
          ty.unwrap_or(AstAttrType::Unknown),
          Nodes::empty(),
          name,
        ),
      );
      attributes.push_back(Node::from_raw(node));
    } else {
      let open = *self.lexer.current();
      self.next_lexeme();

      if self.lexer.current().r#type != Type::RBRACKET {
        loop {
          let name = self.parse_name("attribute name");
          let name_loc = name.location;
          let attr_name = name.name;

          let ct = self.lexer.current().r#type;
          if ct == Type::RAW_STRING
            || ct == Type::QUOTED_STRING
            || ct == Type::LBRACE
            || ct == Type::LPAREN
          {
            let (args, args_location, _expr_location) = self.parse_call_list(None);

            for arg_expr in args.iter_nodes() {
              // parse_call_list 的实参由 arena 分配，`AstArray::iter_nodes` 直接把
              // 元素读成存活节点的共享引用（判空/解引用收口在该门面）。
              if !is_constant_literal(arg_expr) && !is_literal_table(arg_expr) {
                self.report(
                  args_location,
                  format_args!("Only literals can be passed as arguments for attributes"),
                );
              }
            }

            // 元素已在上方循环按 arena 恒非空口径处理，收口为句柄数组入字段。
            let args = Nodes::from_raw_slice(args.as_slice());

            let ty =
              self.validate_attribute(name_loc, attr_name.as_str_or_empty(), attributes, &args);

            let node = self.alloc(
              AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
                Location::new(name_loc.begin, args_location.end),
                ty.unwrap_or(AstAttrType::Unknown),
                args,
                attr_name,
              ),
            );
            attributes.push_back(Node::from_raw(node));
          } else {
            let ty = self.validate_attribute(
              name_loc,
              attr_name.as_str_or_empty(),
              attributes,
              &Nodes::empty(),
            );
            let node = self.alloc(
              AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
                name_loc,
                ty.unwrap_or(AstAttrType::Unknown),
                Nodes::empty(),
                attr_name,
              ),
            );
            attributes.push_back(Node::from_raw(node));
          }

          if self.lexer.current().r#type == Type::COMMA {
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
        let node = self.alloc(
          AstAttr::ast_attr_location_type_item_ast_array_ast_expr_ast_name(
            Location::new(open.location.begin, end_loc.end),
            AstAttrType::Unknown,
            Nodes::empty(),
            self.name_error,
          ),
        );
        attributes.push_back(Node::from_raw(node));
      }

      self.expect_match_and_consume(']', &MatchLexeme::new(&open), false);
    }
  }
}
