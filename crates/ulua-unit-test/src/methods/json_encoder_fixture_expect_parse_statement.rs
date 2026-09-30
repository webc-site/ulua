use ulua_ast::records::ast_stat::AstStat;

use crate::records::json_encoder_fixture::JsonEncoderFixture;

impl JsonEncoderFixture {
  pub fn expect_parse_statement(&mut self, src: &str) -> *mut AstStat {
    let root = self.expect_parse(src);
    let root_ref = unsafe {
      // Safety: expect_parse 成功（失败 panic）返回 fixture arena 存活 AstStatBlock*，&* 物化只读借用读 body，与 cpp 测试 root-> 同形。
      &*root
    };
    ulua_common::LUAU_ASSERT!(1 == root_ref.body.len());
    root_ref.body[0].as_ptr()
  }
}
