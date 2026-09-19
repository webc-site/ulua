use core::{ffi::c_char, ptr::null_mut};

use crate::{
  enums::quote_style_ast::QuoteStyle::Unquoted,
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_table::{
      AstExprTable, Item,
      ItemKind::{General, List, Record},
    },
    ast_node::AstNode,
    cst_expr_table::{CstExprTable, CstExprTableItem},
    cst_node::CstNode,
    lexeme::Type,
    location::Location,
    match_lexeme::MatchLexeme,
    parser::Parser,
    position::Position,
    temp_vector::TempVector,
  },
  rtti::ast_node_as,
};

impl Parser {
  pub fn parse_table_constructor(&mut self) -> *mut AstExpr {
    let mut items = TempVector::new(&mut self.scratch_item);
    let mut cst_items = TempVector::new(&mut self.scratch_cst_item);

    let start = self.lexer.current().location;

    let match_brace = MatchLexeme::new(self.lexer.current());
    self.expect_and_consume_char('{', "table literal");

    while self.lexer.current().r#type != Type(b'}' as i32) {
      if self.lexer.current().r#type == Type(b'[' as i32) {
        let indexer_open_position = self.lexer.current().location.begin;
        let match_location_bracket = MatchLexeme::new(self.lexer.current());
        self.next_lexeme();

        let key = self.parse_expr(0);

        let closing_bracket_found =
          self.expect_match_and_consume(']', &match_location_bracket, false);
        let indexer_close_position = if closing_bracket_found {
          self.lexer.previous_location().begin
        } else {
          Position::missing()
        };

        let equals_found = self.expect_and_consume_char('=', "table field");
        let equals_position = if equals_found {
          self.lexer.previous_location().begin
        } else {
          Position::missing()
        };

        let value = self.parse_expr(0);

        items.push_back(Item {
          kind: General,
          key,
          value,
        });

        if self.options.store_cst_data {
          let (separator, separator_position) = self.table_separator_position();
          cst_items.push_back(CstExprTableItem {
            indexer_open_position,
            indexer_close_position,
            equals_position,
            separator,
            separator_position,
          });
        }
      } else if self.lexer.current().r#type == Type::NAME
        && self.lexer.lookahead().r#type == Type(b'=' as i32)
      {
        let name = self.parse_name("table field");

        let equals_position = self.lexer.current().location.begin;
        self.expect_and_consume_char('=', "table field");

        // C++ `AstArray(name.name.data, name.name.size)`：标识符直切片
        let name_string = AstArray {
          data: name.name.value as *mut c_char,
          size: name.name.len(),
        };

        let key = unsafe {
          (*self.allocator).alloc(AstExprConstantString::new(
            name.location,
            name_string,
            Unquoted,
          )) as *mut AstExpr
        };
        let value = self.parse_expr(0);

        // 记录字段值为函数时，函数名取字段名（cpp debugname 赋值）
        let func = unsafe { ast_node_as::<AstExprFunction>(value as *mut AstNode) };
        if !func.is_null() {
          unsafe {
            (*func).debugname = name.name;
          }
        }

        items.push_back(Item {
          kind: Record,
          key,
          value,
        });

        if self.options.store_cst_data {
          let (separator, separator_position) = self.table_separator_position();
          cst_items.push_back(CstExprTableItem {
            indexer_open_position: Position::missing(),
            indexer_close_position: Position::missing(),
            equals_position,
            separator,
            separator_position,
          });
        }
      } else {
        let expr = self.parse_expr(0);

        items.push_back(Item {
          kind: List,
          key: null_mut(),
          value: expr,
        });

        if self.options.store_cst_data {
          let (separator, separator_position) = self.table_separator_position();
          cst_items.push_back(CstExprTableItem {
            indexer_open_position: Position::missing(),
            indexer_close_position: Position::missing(),
            equals_position: Position::missing(),
            separator,
            separator_position,
          });
        }
      }

      let current_type = self.lexer.current().r#type;
      if current_type == Type(b',' as i32) || current_type == Type(b';' as i32) {
        self.next_lexeme();
      } else if current_type == Type(b'[' as i32) || current_type == Type::NAME {
        self.report(
          self.lexer.current().location,
          format_args!("Expected ',' after table constructor element"),
        );
      } else if current_type != Type(b'}' as i32) {
        break;
      }
    }

    let mut end = self.lexer.current().location;

    if !self.expect_match_and_consume('}', &match_brace, false) {
      end = *self.lexer.previous_location();
    }

    let items_array = self.copy_temp_vector_t(&items);

    let node = unsafe {
      (*self.allocator).alloc(AstExprTable::new(
        Location::new(start.begin, end.end),
        items_array,
      ))
    };

    if self.options.store_cst_data {
      let cst_items_array = self.copy_temp_vector_t(&cst_items);
      let cst_node = unsafe { (*self.allocator).alloc(CstExprTable::new(cst_items_array)) };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstExpr
  }
}
