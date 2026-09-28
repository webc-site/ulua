use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{ast_expr_constant_number::AstExprConstantNumber, ast_visitor::AstVisitor},
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintIntegerParsing<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintIntegerParsing<'ctx> {
  fn visit_expr_constant_number(&mut self, node: &mut AstExprConstantNumber) -> bool {
    lint_integer_parsing_visit(self, node)
  }
}

// —— 原 methods/lint_integer_parsing_process.rs ——
impl<'ctx> LintIntegerParsing<'ctx> {
  lint_stat_process!(LintIntegerParsing);
}

// —— 原 methods/lint_integer_parsing_visit.rs ——
/// 对应 C++ `LintIntegerParsingVisitor::visit(AstExprConstantNumber*)`：按字面量的
/// `parse_result` 发一条 IntegerParsing 警告，恒返回 `true`（继续遍历子节点）。
///
/// `node` 为分析期存活、由 arena 持有的字面量节点共享借用（cpp 裸指针形参的
/// Rust 对应），本函数仅读取其 `parse_result` 与 `base.base.location` 两个 Copy 字段。
pub fn lint_integer_parsing_visit(
  this: &mut LintIntegerParsing,
  node: &AstExprConstantNumber,
) -> bool {
  let (parse_result, location) = (node.parse_result, node.base.base.location);
  let mut context = this.context;
  match parse_result {
    ConstantNumberParseResult::Ok | ConstantNumberParseResult::Malformed => {}
    ConstantNumberParseResult::Imprecise => {
      emit_warning(
        context.get(),
        Code::IntegerParsing,
        location,
        format_args!(
          "Number literal exceeded available precision and was truncated to closest representable number"
        ),
      );
    }
    ConstantNumberParseResult::BinOverflow => {
      emit_warning(
        context.get(),
        Code::IntegerParsing,
        location,
        format_args!(
          "Binary number literal exceeded available precision and was truncated to 2^64"
        ),
      );
    }
    ConstantNumberParseResult::HexOverflow => {
      emit_warning(
        context.get(),
        Code::IntegerParsing,
        location,
        format_args!(
          "Hexadecimal number literal exceeded available precision and was truncated to 2^64"
        ),
      );
    }
    ConstantNumberParseResult::IntOverflow => {
      emit_warning(
        context.get(),
        Code::IntegerParsing,
        location,
        format_args!("Integer number literal was clamped because it was out of range"),
      );
    }
  }
  true
}
