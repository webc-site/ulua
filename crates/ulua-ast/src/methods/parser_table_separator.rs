use crate::{
  enums::type_lexer::Type,
  records::{cst_expr_table::CstExprTableSeparator, parser::Parser},
};

impl Parser {
  pub fn table_separator(&mut self) -> CstExprTableSeparator {
    if self.lexer.current().r#type == Type(',' as i32) {
      CstExprTableSeparator::Comma
    } else if self.lexer.current().r#type == Type(';' as i32) {
      CstExprTableSeparator::Semicolon
    } else {
      CstExprTableSeparator::Missing
    }
  }
}

pub fn parser_table_separator(this: &mut Parser) -> CstExprTableSeparator {
  this.table_separator()
}
