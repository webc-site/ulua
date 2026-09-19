use crate::{
  enums::type_lexer::Type,
  records::{cst_expr_table::CstExprTableSeparator, parser::Parser, position::Position},
};

impl Parser {
  /// 分隔符与其位置：Missing 时位置为零值（C++ `Position()`），否则取当前
  /// 词法单元起点。元组返回值替代 out 参数；table_type / table_constructor
  /// 共用（原两文件各 3 处重复的 separator_position 条件块）。
  pub fn table_separator_position(&mut self) -> (CstExprTableSeparator, Position) {
    let separator = if self.lexer.current().r#type == Type(',' as i32) {
      CstExprTableSeparator::Comma
    } else if self.lexer.current().r#type == Type(';' as i32) {
      CstExprTableSeparator::Semicolon
    } else {
      CstExprTableSeparator::Missing
    };
    let separator_position = if separator == CstExprTableSeparator::Missing {
      Position::missing()
    } else {
      self.lexer.current().location.begin
    };
    (separator, separator_position)
  }
}
