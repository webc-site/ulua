use ulua_ast::records::ast_stat::AstStat;

use crate::records::json_encoder_fixture::JsonEncoderFixture;

impl JsonEncoderFixture {
  /// cpp `expectParseStatement`：根块体首条语句的节点身份指针。
  ///
  /// 返回值只作身份桥（喂给 `json(impl AstNodePtr)` 的 encoder 边界），
  /// 不在测试侧解引用；解引用收口在 `Node::get` 与 analysis 侧 encoder。
  pub fn expect_parse_statement(&mut self, src: &str) -> *mut AstStat {
    let root = self.expect_parse(src);
    let root_ref = root.get();
    ulua_common::LUAU_ASSERT!(1 == root_ref.body.len());
    root_ref.body[0].as_ptr()
  }
}
