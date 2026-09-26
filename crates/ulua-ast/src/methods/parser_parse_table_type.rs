use core::ptr::NonNull;

use crate::{
  enums::{
    ast_table_access::AstTableAccess, quote_style_cst::QuoteStyle::QuotedDouble, type_lexer::Type,
  },
  functions::optional_node::{node_opt, opt_node},
  records::{
    ast_array::AstArray,
    ast_name::AstName,
    ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp,
    ast_type::AstType,
    ast_type_reference::AstTypeReference,
    ast_type_table::AstTypeTable,
    cst_expr_constant_string::CstExprConstantString,
    cst_type_table::{CstTypeTable, CstTypeTableItem, CstTypeTableItemKind},
    lexeme::{Lexeme, lexeme_name_is},
    location::Location,
    match_lexeme::MatchLexeme,
    parser::Parser,
    position::Position,
    temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_table_type(&mut self, in_declaration_context: bool) -> *mut AstType {
    self.increment_recursion_counter("type annotation");

    let mut props = TempVector::new(&mut self.scratch_table_type_props);
    let mut cst_items = TempVector::new(&mut self.scratch_cst_table_type_props);
    let mut indexer: Option<NonNull<AstTableIndexer>> = None;

    let start = self.lexer.current().location;

    let match_brace = MatchLexeme::new(self.lexer.current());
    self.expect_and_consume_char('{', "table type");

    let mut is_array = false;

    while self.lexer.current().r#type != Type::RBRACE {
      let mut access = AstTableAccess::ReadWrite;
      let mut access_location = None;

      // `read`/`write` 访问标注：仅 NAME 且后随非 `:`（否则是字段名）。
      // lexeme_name_is 内合并 NAME 判型与名字比较，免 data.name 裸指针解包。
      if self.lexer.current().r#type == Type::NAME && self.lexer.lookahead().r#type != Type::COLON {
        // lookahead 仅前视不消费，current 与条件处一致
        if lexeme_name_is(self.lexer.current(), "read") {
          access_location = Some(self.lexer.current().location);
          access = AstTableAccess::Read;
          self.next_lexeme();
        } else if lexeme_name_is(self.lexer.current(), "write") {
          access_location = Some(self.lexer.current().location);
          access = AstTableAccess::Write;
          self.next_lexeme();
        }
      }

      if self.lexer.current().r#type == Type::LBRACKET {
        let begin = *self.lexer.current();
        self.next_lexeme(); // [

        if (self.lexer.current().r#type == Type::RAW_STRING
          || self.lexer.current().r#type == Type::QUOTED_STRING)
          && self.lexer.lookahead().r#type == Type::RBRACKET
        {
          let mut style = QuotedDouble;
          let mut block_depth = 0;
          if self.options.store_cst_data {
            let (s, bd) = self.extract_string_details();
            style = s;
            block_depth = bd;
          }

          let string_position = self.lexer.current().location.begin;
          let (chars, source_string) = match self.parse_char_array(self.options.store_cst_data) {
            Some((value, original)) => (Some(value), original),
            None => (None, AstArray::EMPTY),
          };

          let begin_match = Lexeme::new(begin.location, begin.r#type);
          let indexer_close_position =
            self.expect_match_and_consume_position(']', &MatchLexeme::new(&begin_match), false);
          let colon_position = self.expect_and_consume_char_position(':', "table field");

          let r#type = self.parse_type(false);

          // since AstName contains a char*, it can't contain null
          let contains_null = chars.as_ref().is_some_and(|c| c.contains_null());

          if let (Some(chars_unwrapped), false) = (chars, contains_null) {
            props.push_back(AstTableProp {
              name: AstName {
                value: chars_unwrapped.data as *const u8,
                len: chars_unwrapped.size as u32,
              },
              location: begin.location,
              r#type,
              access,
              access_location,
            });
            if self.options.store_cst_data {
              let (separator, separator_position) = self.table_separator_position();
              let string_info = self.alloc(CstExprConstantString::new(
                source_string,
                style,
                block_depth,
              ));
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
            self.report(
              begin.location,
              format_args!("String literal contains malformed escape sequence or \\0"),
            );
          }
        } else {
          // 第二个 indexer 报错；首个记录到 AST/CST（原 if/else 两侧重复
          // parse_table_indexer 调用，合并为单次调用后按已有 indexer 分派）
          let table_indexer_result = self.parse_table_indexer(access, access_location, begin);
          if indexer.is_some() {
            // Safety: table_indexer_result.node 由 parse_table_indexer 内 `self.alloc(AstTableIndexer{..})`
            // 产出（arena alloc 恒非空，失败 handle_alloc_error 中止；bump 块地址永不移动），此处仅只读
            // 拷贝该记录的直接 `location` 字段，不构造 `&mut`，单线程串行无别名冲突。
            let bad_indexer_location = unsafe { (*table_indexer_result.node).location };
            self.report(
              bad_indexer_location,
              format_args!("Cannot have more than one table indexer"),
            );
          } else {
            indexer = node_opt(table_indexer_result.node);
            if self.options.store_cst_data {
              let (separator, separator_position) = self.table_separator_position();
              cst_items.push_back(CstTypeTableItem {
                kind: CstTypeTableItemKind::Indexer,
                indexer_open_position: table_indexer_result.indexer_open_position,
                indexer_close_position: table_indexer_result.indexer_close_position,
                colon_position: table_indexer_result.colon_position,
                separator,
                separator_position,
                string_info: opt_node(None),
                string_position: Position::missing(),
              });
            }
          }
        }
      } else if props.is_empty()
        && indexer.is_none()
        && !(self.lexer.current().r#type == Type::NAME
          && self.lexer.lookahead().r#type == Type::COLON)
      {
        let r#type = self.parse_type(false);
        is_array = true;

        // array-like table type: {T} desugars into {[number]: T}（cpp 无 flag，无条件）
        let null_type_location = Location::with_length(start.begin, 0);
        let index = self.alloc_type(AstTypeReference::new(
          null_type_location,
          None,
          self.name_number,
          None,
          null_type_location,
          false,
          AstArray::EMPTY,
        ));
        indexer = node_opt(self.alloc(AstTableIndexer {
          index_type: index,
          result_type: r#type,
          location: unsafe {
            // Safety: r#type 为 180 行 parse_type 刚 arena 分配的存活类型节点（恒非空，失败 handle_alloc_error 中止），(*r#type).base.location 为 repr(C) 基类前缀只读拷贝，随即写入新建 indexer。
            (*r#type).base.location
          },
          access,
          access_location,
        }));

        break;
      } else {
        let Some(name_unwrapped) = self.parse_name_opt("table field") else {
          break;
        };

        let colon_position = self.expect_and_consume_char_position(':', "table field");

        let r#type = self.parse_type(in_declaration_context);

        props.push_back(AstTableProp {
          name: name_unwrapped.name,
          location: name_unwrapped.location,
          r#type,
          access,
          access_location,
        });

        if self.options.store_cst_data {
          let (separator, separator_position) = self.table_separator_position();
          cst_items.push_back(CstTypeTableItem {
            kind: CstTypeTableItemKind::Property,
            indexer_open_position: Position::missing(),
            indexer_close_position: Position::missing(),
            colon_position,
            separator,
            separator_position,
            string_info: opt_node(None),
            string_position: Position::missing(),
          });
        }
      }

      if self.lexer.current().r#type == Type::COMMA
        || self.lexer.current().r#type == Type::SEMICOLON
      {
        self.next_lexeme();
      } else if self.lexer.current().r#type != Type::RBRACE {
        break;
      }
    }

    let mut end = self.lexer.current().location;

    if !self.expect_match_and_consume('}', &match_brace, true) {
      end = *self.lexer.previous_location();
    }

    let props_array = self.copy_temp_vector_t(&props);
    let node = self.alloc_type(AstTypeTable::new(
      Location::new(start.begin, end.end),
      props_array,
      opt_node(indexer),
    ));

    if self.options.store_cst_data {
      let cst_items_array = self.copy_temp_vector_t(&cst_items);
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstTypeTable::new(cst_items_array, is_array))
      });
    }

    node
  }
}
