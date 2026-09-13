use core::ptr::null_mut;

use crate::{
  enums::{
    ast_table_access::AstTableAccess,
    quote_style_cst::QuoteStyle::QuotedDouble,
    separator::Separator::{Comma, Missing, Semicolon},
  },
  records::{
    ast_array::AstArray,
    ast_name::AstName,
    ast_node::AstNode,
    ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp,
    ast_type::AstType,
    ast_type_reference::AstTypeReference,
    ast_type_table::AstTypeTable,
    cst_expr_constant_string::CstExprConstantString,
    cst_expr_table::Separator,
    cst_node::CstNode,
    cst_type_table::{CstTypeTable, CstTypeTableItem, CstTypeTableItemKind},
    lexeme::{Lexeme, Type},
    location::Location,
    match_lexeme::MatchLexeme,
    parser::Parser,
    position::Position,
    temp_vector::TempVector,
  },
  rtti::CstNodeClass,
};

impl Parser {
  pub fn parse_table_type(&mut self, in_declaration_context: bool) -> *mut AstType {
    self.increment_recursion_counter("type annotation");

    let mut props = TempVector::new(&mut self.scratch_table_type_props);
    let mut cst_items = TempVector::new(&mut self.scratch_cst_table_type_props);
    let mut indexer: *mut AstTableIndexer = null_mut();

    let start = self.lexer.current().location;

    let match_brace = MatchLexeme::new(self.lexer.current());
    self.expect_and_consume_char('{', "table type");

    let mut is_array = false;

    while self.lexer.current().r#type != Type('}' as i32) {
      let mut access = AstTableAccess::ReadWrite;
      let mut access_location = None;

      if self.lexer.current().r#type == Type::NAME
        && self.lexer.lookahead().r#type != Type(':' as i32)
      {
        let current_name = unsafe { self.lexer.current().data.name };
        if AstName::ast_name_c_char(current_name).operator_eq_c_char(c"read") {
          access_location = Some(self.lexer.current().location);
          access = AstTableAccess::Read;
          self.next_lexeme();
        } else if AstName::ast_name_c_char(current_name).operator_eq_c_char(c"write") {
          access_location = Some(self.lexer.current().location);
          access = AstTableAccess::Write;
          self.next_lexeme();
        }
      }

      if self.lexer.current().r#type == Type('[' as i32) {
        let begin = *self.lexer.current();
        self.next_lexeme(); // [

        if (self.lexer.current().r#type == Type::RAW_STRING
          || self.lexer.current().r#type == Type::QUOTED_STRING)
          && self.lexer.lookahead().r#type == Type(']' as i32)
        {
          let mut style = QuotedDouble;
          let mut block_depth = 0;
          if self.options.store_cst_data {
            let (s, bd) = self.extract_string_details();
            style = s;
            block_depth = bd;
          }

          let string_position = self.lexer.current().location.begin;
          let mut source_string = AstArray::default();
          let chars = self.parse_char_array(if self.options.store_cst_data {
            Some(&mut source_string)
          } else {
            None
          });

          let begin_match = Lexeme::new(begin.location, begin.r#type);
          let closing_bracket_found =
            self.expect_match_and_consume(']', &MatchLexeme::new(&begin_match), false);
          let indexer_close_position = if closing_bracket_found {
            self.lexer.previous_location().begin
          } else {
            Position::missing()
          };

          let colon_found = self.expect_and_consume_char(':', "table field");
          let colon_position = if colon_found {
            self.lexer.previous_location().begin
          } else {
            Position::missing()
          };

          let r#type = self.parse_type_bool(false);

          // since AstName contains a char*, it can't contain null
          let contains_null = chars.as_ref().is_some_and(|c| c.as_slice().contains(&0));

          if let (Some(chars_unwrapped), false) = (chars, contains_null) {
            props.push_back(AstTableProp {
              name: AstName {
                value: chars_unwrapped.data,
              },
              location: begin.location,
              r#type,
              access,
              access_location,
            });
            if self.options.store_cst_data {
              let sep = self.table_separator();
              let separator = match sep {
                Comma => Separator::Comma,
                Semicolon => Separator::Semicolon,
                Missing => Separator::Missing,
              };
              let separator_position = if separator != Separator::Missing {
                self.lexer.current().location.begin
              } else {
                Position::missing()
              };
              let string_info = unsafe {
                (*self.allocator).alloc(CstExprConstantString {
                  base: CstNode {
                    class_index: CstExprConstantString::CLASS_INDEX,
                  },
                  source_string,
                  quote_style: style,
                  block_depth,
                })
              };
              cst_items.push_back(CstTypeTableItem {
                kind: CstTypeTableItemKind::StringProperty,
                indexer_open_position: begin.location.begin,
                indexer_close_position,
                colon_position,
                separator,
                separator_position,
                string_info,
                string_position,
              });
            }
          } else {
            self.report_location_c_char_item(
              begin.location,
              format_args!("String literal contains malformed escape sequence or \\0"),
            );
          }
        } else {
          if !indexer.is_null() {
            let table_indexer_result = self.parse_table_indexer(access, access_location, begin);
            let bad_indexer = table_indexer_result.node;
            self.report_location_c_char_item(
              unsafe { (*bad_indexer).location },
              format_args!("Cannot have more than one table indexer"),
            );
          } else {
            let table_indexer_result = self.parse_table_indexer(access, access_location, begin);
            indexer = table_indexer_result.node;
            if self.options.store_cst_data {
              let sep = self.table_separator();
              let separator = match sep {
                Comma => Separator::Comma,
                Semicolon => Separator::Semicolon,
                Missing => Separator::Missing,
              };
              let separator_position = if separator != Separator::Missing {
                self.lexer.current().location.begin
              } else {
                Position::missing()
              };
              cst_items.push_back(CstTypeTableItem {
                kind: CstTypeTableItemKind::Indexer,
                indexer_open_position: table_indexer_result.indexer_open_position,
                indexer_close_position: table_indexer_result.indexer_close_position,
                colon_position: table_indexer_result.colon_position,
                separator,
                separator_position,
                string_info: null_mut(),
                string_position: Position::missing(),
              });
            }
          }
        }
      } else if props.empty()
        && indexer.is_null()
        && !(self.lexer.current().r#type == Type::NAME
          && self.lexer.lookahead().r#type == Type(':' as i32))
      {
        let r#type = self.parse_type_bool(false);
        is_array = true;

        // array-like table type: {T} desugars into {[number]: T}（cpp 无 flag，无条件）
        let null_type_location = Location::with_length(start.begin, 0);
        let index = unsafe {
          (*self.allocator).alloc(AstTypeReference::new(
            null_type_location,
            None,
            self.name_number,
            None,
            null_type_location,
            false,
            AstArray::default(),
          ))
        } as *mut AstType;
        indexer = unsafe {
          (*self.allocator).alloc(AstTableIndexer {
            index_type: index,
            result_type: r#type,
            location: (*r#type).base.location,
            access,
            access_location,
          })
        };

        break;
      } else {
        let name = self.parse_name_opt("table field");

        if name.is_none() {
          break;
        }

        let name_unwrapped = name.unwrap();

        let colon_found = self.expect_and_consume_char(':', "table field");
        let colon_position = if colon_found {
          self.lexer.previous_location().begin
        } else {
          Position::missing()
        };

        let r#type = self.parse_type_bool(in_declaration_context);

        props.push_back(AstTableProp {
          name: name_unwrapped.name,
          location: name_unwrapped.location,
          r#type,
          access,
          access_location,
        });

        if self.options.store_cst_data {
          let sep = self.table_separator();
          let separator = match sep {
            Comma => Separator::Comma,
            Semicolon => Separator::Semicolon,
            Missing => Separator::Missing,
          };
          let separator_position = if separator != Separator::Missing {
            self.lexer.current().location.begin
          } else {
            Position::missing()
          };

          cst_items.push_back(CstTypeTableItem {
            kind: CstTypeTableItemKind::Property,
            indexer_open_position: Position::missing(),
            indexer_close_position: Position::missing(),
            colon_position,
            separator,
            separator_position,
            string_info: null_mut(),
            string_position: Position::missing(),
          });
        }
      }

      if self.lexer.current().r#type == Type(',' as i32)
        || self.lexer.current().r#type == Type(';' as i32)
      {
        self.next_lexeme();
      } else {
        if self.lexer.current().r#type != Type('}' as i32) {
          break;
        }
      }
    }

    let mut end = self.lexer.current().location;

    if !self.expect_match_and_consume('}', &match_brace, true) {
      end = *self.lexer.previous_location();
    }

    let props_array = self.copy_temp_vector_t(&props);
    let node = unsafe {
      (*self.allocator).alloc(AstTypeTable::new(
        Location::new(start.begin, end.end),
        props_array,
        indexer,
      ))
    };

    if self.options.store_cst_data {
      let cst_items_array = self.copy_temp_vector_t(&cst_items);
      let cst_node =
        unsafe { (*self.allocator).alloc(CstTypeTable::new(cst_items_array, is_array)) };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstType
  }
}
