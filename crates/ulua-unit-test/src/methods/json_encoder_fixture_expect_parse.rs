use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::json_encoder_fixture::JsonEncoderFixture;

impl JsonEncoderFixture {
  pub fn expect_parse(&mut self, src: &str) -> *mut AstStatBlock {
    let parse_result = self.parse(src);
    ulua_common::LUAU_ASSERT!(parse_result.errors.is_empty());
    parse_result.root
  }
}
