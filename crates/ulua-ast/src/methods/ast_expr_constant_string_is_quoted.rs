use crate::{
  enums::quote_style_ast::QuoteStyle, records::ast_expr_constant_string::AstExprConstantString,
};

impl AstExprConstantString {
  pub fn is_quoted(&self) -> bool {
    matches!(
      self.quote_style,
      QuoteStyle::QuotedSimple | QuoteStyle::QuotedRaw
    )
  }
}

pub fn ast_expr_constant_string_is_quoted(this: &AstExprConstantString) -> bool {
  this.is_quoted()
}
