use core::{mem::take, str::from_utf8};

use ulua_common::fflag;

use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  functions::{parse_double::parse_double, parse_integer_64::parse_integer_64},
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_number::AstExprConstantNumber,
    cst_expr_constant_integer::CstExprConstantInteger,
    cst_expr_constant_number::CstExprConstantNumber, parser::Parser,
  },
};

impl Parser {
  pub fn parse_number(&mut self) -> *mut AstExpr {
    // cpp `scratchData.assign(lexer.current().data, lexer.current().getLength())`：
    // NUMBER 词素的字节区间经全仓唯一切片门面 [`Lexeme::data_bytes`]
    // （records/lexeme.rs，联合体位拷贝契约同处收口）取得，本文件不再裸建切片。
    // 行为 ⇔ 原「data 臂直读 + get_length」：本入口仅由 `parse_simple_expr` 在
    // `Type::NUMBER` 判定后调用（cpp Parser.cpp case Number 同款分发），NUMBER 在
    // 门面变体集内；词法器成对写入保证非空指针，理论空指针/空词素退化为空串 ⇔
    // cpp `assign(p, 0)` 不读字节。
    let current = *self.lexer.current();
    let start = current.location;
    self.scratch_data.clear();
    self
      .scratch_data
      .push_str(from_utf8(current.data_bytes().unwrap_or_default()).unwrap_or(""));

    let mut source_data = AstArray::EMPTY;
    if self.options.store_cst_data {
      // copy_string 需 `&mut self`，而源串借自 self.scratch_data（E0502）。
      // `mem::take` 把 String 临时移出再还回，零拷贝绕开借用冲突，
      // 取代原先整串 heap clone。
      let sd = take(&mut self.scratch_data);
      source_data = self.copy_bytes(sd.as_bytes());
      self.scratch_data = sd;
    }

    // Remove all internal _
    if self.scratch_data.contains('_') {
      self.scratch_data.retain(|c| c != '_');
    }

    if fflag::LuauIntegerType2.get() && self.scratch_data.ends_with('i') {
      let (result, value) =
        if self.scratch_data.starts_with("0x") || self.scratch_data.starts_with("0X") {
          parse_integer_64(&self.scratch_data, 16)
        } else if self.scratch_data.starts_with("0b") || self.scratch_data.starts_with("0B") {
          parse_integer_64(&self.scratch_data[2..], 2)
        } else {
          parse_integer_64(&self.scratch_data, 10)
        };

      self.next_lexeme();

      if result == ConstantNumberParseResult::Malformed {
        return self.report_expr_error(start, AstArray::EMPTY, format_args!("Malformed integer"));
      }

      if result != ConstantNumberParseResult::Ok {
        return self.report_expr_error(start, AstArray::EMPTY, format_args!("Integer overflow"));
      }

      let node = self.alloc_expr(AstExprConstantInteger::new(start, value, result));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprConstantInteger::new(source_data))
      });

      node
    } else {
      let (result, value) = parse_double(&self.scratch_data);

      self.next_lexeme();

      if result == ConstantNumberParseResult::Malformed {
        return self.report_expr_error(start, AstArray::EMPTY, format_args!("Malformed number"));
      }

      let node = self.alloc_expr(AstExprConstantNumber::new(start, value, result));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprConstantNumber::new(source_data))
      });

      node
    }
  }
}
