use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{
    quote_style_ast::QuoteStyle::{QuotedRaw, QuotedSimple},
    quote_style_cst::QuoteStyle::QuotedDouble,
    type_lexer::Type,
  },
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    cst_expr_constant_string::CstExprConstantString, location::Location, parser::Parser,
  },
};

impl Parser {
  pub fn parse_string(&mut self) -> *mut AstExpr {
    let location: Location = self.lexer.current().location;

    let style = match self.lexer.current().r#type {
      Type::QUOTED_STRING | Type::INTERP_STRING_SIMPLE => QuotedSimple,
      Type::RAW_STRING => QuotedRaw,
      _ => {
        LUAU_ASSERT!(false);
        QuotedSimple
      }
    };

    let mut full_style = QuotedDouble;
    let mut block_depth: u32 = 0;

    if self.options.store_cst_data {
      let (fs, bd) = self.extract_string_details();
      full_style = fs;
      block_depth = bd;
    }

    if let Some((value, original_string)) = self.parse_char_array(self.options.store_cst_data) {
      let node = self.alloc_expr(AstExprConstantString::new(location, value, style));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprConstantString::new(
          original_string,
          full_style,
          block_depth,
        ))
      });

      node
    } else {
      self.report_expr_error(
        location,
        AstArray::EMPTY,
        format_args!("String literal contains malformed escape sequence"),
      )
    }
  }
}
