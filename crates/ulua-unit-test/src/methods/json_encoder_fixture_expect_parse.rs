use core::ptr::NonNull;

use ulua_ast::records::{ast_stat_block::AstStatBlock, node_handle::Node};

use crate::records::json_encoder_fixture::JsonEncoderFixture;

impl JsonEncoderFixture {
  /// cpp `expectParse`：断言解析零错误后交出根块句柄（成功解析恒有根块）。
  pub fn expect_parse(&mut self, src: &str) -> Node<AstStatBlock> {
    let parse_result = self.parse(src);
    ulua_common::LUAU_ASSERT!(parse_result.errors.is_empty());
    Node::from_non_null(
      NonNull::new(parse_result.root).expect("成功解析恒有根块（失败路径已 panic）"),
    )
  }
}
