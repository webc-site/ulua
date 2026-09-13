use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::ast_expr_constant_number::AstExprConstantNumber,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_integer_parsing::LintIntegerParsing,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn lint_integer_parsing_visit(
  this: &mut LintIntegerParsing,
  node: *mut AstExprConstantNumber,
) -> bool {
  match unsafe { &(*node).parse_result } {
    ConstantNumberParseResult::Ok | ConstantNumberParseResult::Malformed => {}
    ConstantNumberParseResult::Imprecise => {
      emit_warning(
        unsafe { &mut *this.context },
        Code::IntegerParsing,
        unsafe { (*node).base.base.location },
        format_args!(
          "Number literal exceeded available precision and was truncated to closest representable number"
        ),
      );
    }
    ConstantNumberParseResult::BinOverflow => {
      emit_warning(
        unsafe { &mut *this.context },
        Code::IntegerParsing,
        unsafe { (*node).base.base.location },
        format_args!(
          "Binary number literal exceeded available precision and was truncated to 2^64"
        ),
      );
    }
    ConstantNumberParseResult::HexOverflow => {
      emit_warning(
        unsafe { &mut *this.context },
        Code::IntegerParsing,
        unsafe { (*node).base.base.location },
        format_args!(
          "Hexadecimal number literal exceeded available precision and was truncated to 2^64"
        ),
      );
    }
    ConstantNumberParseResult::IntOverflow => {
      emit_warning(
        unsafe { &mut *this.context },
        Code::IntegerParsing,
        unsafe { (*node).base.base.location },
        format_args!("Integer number literal was clamped because it was out of range"),
      );
    }
  }

  true
}
