use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::records::{lexeme::Type, parser::Parser};

impl Parser {
  pub(crate) fn expect_and_consume_fail_with_lookahead(
    &mut self,
    type_: Type,
    context: &str,
  ) -> bool {
    LUAU_NOINLINE! {
        fn inner(parser: &mut Parser, type_: Type, context: &str) -> bool {
            parser.expect_and_consume_fail(type_, context);

            // check if this is an extra token and the expected token is next
            if parser.lexer.lookahead().r#type == type_ {
                // skip invalid and consume expected
                parser.next_lexeme();
                parser.next_lexeme();
            }

            false
        }
    }

    inner(self, type_, context)
  }
}
