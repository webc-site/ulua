use crate::{
  enums::{quote_style_ast::QuoteStyle::Unquoted, type_lexer::Type},
  functions::optional_node::opt_node,
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_table::{
      AstExprTable, Item,
      ItemKind::{General, List, Record},
    },
    cst_expr_table::{CstExprTable, CstExprTableItem},
    location::Location,
    match_lexeme::MatchLexeme,
    parser::Parser,
    position::Position,
    temp_vector::TempVector,
  },
  rtti::ast_node_try_as_ptr_mut,
};

impl Parser {
  pub fn parse_table_constructor(&mut self) -> *mut AstExpr {
    let mut items = TempVector::new(&mut self.scratch_item);
    let mut cst_items = TempVector::new(&mut self.scratch_cst_item);

    let start = self.lexer.current().location;

    let match_brace = MatchLexeme::new(self.lexer.current());
    self.expect_and_consume_char('{', "table literal");

    while self.lexer.current().r#type != Type::RBRACE {
      if self.lexer.current().r#type == Type::LBRACKET {
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
        && self.lexer.lookahead().r#type == Type::EQUAL_SIGN
      {
        let name = self.parse_name("table field");

        let equals_position = self.lexer.current().location.begin;
        self.expect_and_consume_char('=', "table field");

        // C++ `AstArray(name.name.data, name.name.size)`：标识符直切片
        // （批 2 存储面：name.value 与 AstArray<u8> 同字节域）
        let name_string = AstArray {
          data: name.name.value as *mut u8,
          size: name.name.len(),
        };

        let key = self.alloc_expr(AstExprConstantString::new(
          name.location,
          name_string,
          Unquoted,
        ));
        let value = self.parse_expr(0);

        // 记录字段值为函数时，函数名取字段名（cpp debugname 赋值）
        if let Some(func) = unsafe {
          // Safety: value 为 parse_expr 刚 arena 分配的存活节点（恒非空）；ast_node_try_as_ptr_mut::<AstExprFunction> 收口判空与类索引判型：读偏移 0 的 class_index，未命中返回 None 不写，命中后 repr(C) 基类前缀同址下转，写 debugname 时节点尚未入 items、无其他借用，单线程无别名。
          ast_node_try_as_ptr_mut::<AstExprFunction>(value)
        } {
          func.debugname = name.name;
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
          key: opt_node(None),
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
      if current_type == Type::COMMA || current_type == Type::SEMICOLON {
        self.next_lexeme();
      } else if current_type == Type::LBRACKET || current_type == Type::NAME {
        self.report(
          self.lexer.current().location,
          format_args!("Expected ',' after table constructor element"),
        );
      } else if current_type != Type::RBRACE {
        break;
      }
    }

    let mut end = self.lexer.current().location;

    if !self.expect_match_and_consume('}', &match_brace, false) {
      end = *self.lexer.previous_location();
    }

    let items_array = self.copy_temp_vector_t(&items);

    let node = self.alloc_expr(AstExprTable::new(
      Location::new(start.begin, end.end),
      items_array,
    ));

    if self.options.store_cst_data {
      let cst_items_array = self.copy_temp_vector_t(&cst_items);
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprTable::new(cst_items_array))
      });
    }

    node
  }
}
