use core::{ffi::c_char, slice::from_raw_parts};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::brace_type::BraceType::InterpolatedString,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString,
    ast_node::AstNode, cst_expr_interp_string::CstExprInterpString, cst_node::CstNode,
    lexeme::Type, lexer::Lexer, location::Location, parser::Parser, position::Position,
    temp_vector::TempVector,
  },
};

/// MID/END 后缺表达式的报错文案。
const K_EXPECTED_EXPR_IN_INTERP: &str =
  "Malformed interpolated string, expected expression inside '{}'";
/// 缺反引号的报错文案（本文件两处重复）。
const K_MISSING_BACKTICK: &str = "Malformed interpolated string; did you forget to add a '`'?";

impl Parser {
  pub fn parse_interp_string(&mut self) -> *mut AstExpr {
    let mut strings = TempVector::new(&mut self.scratch_string);
    let mut source_strings = TempVector::new(&mut self.scratch_string_2);
    let mut string_positions = TempVector::new(&mut self.scratch_position);
    let mut expressions = TempVector::new(&mut self.scratch_expr);

    let start_location = self.lexer.current().location;
    let mut end_location;

    loop {
      let current_lexeme = *self.lexer.current();
      LUAU_ASSERT!(
        current_lexeme.r#type == Type::INTERP_STRING_BEGIN
          || current_lexeme.r#type == Type::INTERP_STRING_MID
          || current_lexeme.r#type == Type::INTERP_STRING_END
          || current_lexeme.r#type == Type::INTERP_STRING_SIMPLE
      );

      end_location = current_lexeme.location;

      let length = current_lexeme.get_length() as usize;
      // cpp `data` 联合体直读：INTERP 族词素负载为字符串指针，一次 cast 取字节切片
      let bytes = unsafe { from_raw_parts(current_lexeme.data.data as *const u8, length) };

      if self.options.store_cst_data {
        let source_string = self.copy_bytes(bytes);
        source_strings.push_back(source_string);
        string_positions.push_back(current_lexeme.location.begin);
      }

      let mut data = bytes.to_vec();
      if !Lexer::fixup_quoted_bytes(&mut data) {
        self.next_lexeme();
        return self.report_expr_error(
          Location::new(start_location.begin, end_location.end),
          AstArray::EMPTY,
          format_args!("Interpolated string literal contains malformed escape sequence"),
        ) as *mut AstExpr;
      }

      let chars = self.copy_bytes(&data);
      self.next_lexeme();
      strings.push_back(chars);

      if current_lexeme.r#type == Type::INTERP_STRING_END
        || current_lexeme.r#type == Type::INTERP_STRING_SIMPLE
      {
        break;
      }

      // 表达式检查状态机：MID/END/BROKEN_STRING 时先报错再终止（match 臂
      // 直接产出文案，替代原 bool 旗标的二次判读）；否则解析表达式。
      let error_message = match self.lexer.current().r#type {
        Type::INTERP_STRING_MID | Type::INTERP_STRING_END => Some(K_EXPECTED_EXPR_IN_INTERP),
        Type::BROKEN_STRING => Some(K_MISSING_BACKTICK),
        _ => None,
      };

      if let Some(message) = error_message {
        self.next_lexeme();
        expressions.push_back(self.report_expr_error(
          end_location,
          AstArray::EMPTY,
          format_args!("{message}"),
        ) as *mut AstExpr);
        break;
      }

      expressions.push_back(self.parse_expr(0));

      match self.lexer.current().r#type {
        Type::INTERP_STRING_BEGIN | Type::INTERP_STRING_MID | Type::INTERP_STRING_END => {}
        Type::BROKEN_INTERP_DOUBLE_BRACE => {
          self.next_lexeme();
          return self.report_expr_error(
            end_location,
            AstArray::EMPTY,
            format_args!(
              "Double braces are not permitted within interpolated strings; did you mean '\\{{'?"
            ),
          ) as *mut AstExpr;
        }
        Type::BROKEN_STRING | Type::EOF => {
          if self.lexer.current().r#type == Type::BROKEN_STRING {
            self.next_lexeme();
          }
          // SAFETY: build_interp_string 收口节点与 CST 构造
          let node = self.build_interp_string(
            start_location.begin,
            self.lexer.previous_location().end,
            &strings,
            &expressions,
            &source_strings,
            &string_positions,
          );

          // 栈顶非插串（含栈空）时报缺反引号：原两分支报告完全相同，合并判据
          // 栈顶为插串时报缺 '}}'，栈空时报缺 '`'，其余不报（原三分支判据）
          match self.lexer.peek_brace_stack_top() {
            Some(InterpolatedString) => self.report(
              *self.lexer.previous_location(),
              format_args!("Malformed interpolated string; did you forget to add a '}}'?"),
            ),
            None => self.report(
              *self.lexer.previous_location(),
              format_args!("{K_MISSING_BACKTICK}"),
            ),
            _ => {}
          }

          return node;
        }
        _ => {
          let current = *self.lexer.current();
          return self.report_expr_error(
            end_location,
            AstArray::EMPTY,
            format_args!("Malformed interpolated string, got {current}"),
          ) as *mut AstExpr;
        }
      }
    }

    // SAFETY: build_interp_string 收口节点与 CST 构造
    self.build_interp_string(
      start_location.begin,
      end_location.end,
      &strings,
      &expressions,
      &source_strings,
      &string_positions,
    )
  }

  /// 构造 AstExprInterpString 节点与 CST 对应（原 BROKEN_STRING/EOF 分支与
  /// 循环出口两处重复块，合并于此）。unsafe 仅 allocator 分配与映射插入。
  fn build_interp_string(
    &mut self,
    start: Position,
    end: Position,
    strings: &TempVector<'_, AstArray<c_char>>,
    expressions: &TempVector<'_, *mut AstExpr>,
    source_strings: &TempVector<'_, AstArray<c_char>>,
    string_positions: &TempVector<'_, Position>,
  ) -> *mut AstExpr {
    let strings_array = self.copy_temp_vector_t(strings);
    let expressions_array = self.copy_temp_vector_t(expressions);
    let node = unsafe {
      (*self.allocator).alloc(AstExprInterpString::new(
        Location::new(start, end),
        strings_array,
        expressions_array,
      ))
    };

    if self.options.store_cst_data {
      let source_strings_array = self.copy_temp_vector_t(source_strings);
      let string_positions_array = self.copy_temp_vector_t(string_positions);
      let cst_node = unsafe {
        (*self.allocator).alloc(CstExprInterpString::new(
          source_strings_array,
          string_positions_array,
        ))
      };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstExpr
  }
}
