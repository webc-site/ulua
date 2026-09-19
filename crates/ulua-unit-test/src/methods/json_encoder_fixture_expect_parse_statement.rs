use ulua_ast::records::ast_stat::AstStat;

use crate::records::json_encoder_fixture::JsonEncoderFixture;

impl JsonEncoderFixture {
  pub fn expect_parse_statement(&mut self, src: &str) -> *mut AstStat {
    let root = self.expect_parse(src);
    let root_ref = unsafe { &*root };
    ulua_common::LUAU_ASSERT!(1 == root_ref.body.size);
    unsafe { *root_ref.body.data.add(0) }
  }
}
