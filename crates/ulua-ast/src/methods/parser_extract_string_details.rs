use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::quote_style_cst::QuoteStyle,
  records::{
    lexeme::{QuoteStyle::Double, Type},
    parser::Parser,
  },
};

impl Parser {
  pub fn extract_string_details(&mut self) -> (QuoteStyle, u32) {
    let mut style: QuoteStyle = QuoteStyle::QuotedDouble;
    let mut block_depth: u32 = 0;

    let current = self.lexer.current();

    match current.r#type {
      Type::QUOTED_STRING => {
        style = if current.get_quote_style() == Double {
          QuoteStyle::QuotedDouble
        } else {
          QuoteStyle::QuotedSingle
        };
      }
      Type::INTERP_STRING_SIMPLE => {
        style = QuoteStyle::QuotedInterp;
      }
      Type::RAW_STRING => {
        style = QuoteStyle::QuotedRaw;
        block_depth = current.get_block_depth();
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }

    (style, block_depth)
  }
}
