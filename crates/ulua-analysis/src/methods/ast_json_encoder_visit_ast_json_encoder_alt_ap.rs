//! Source: `Analysis/src/AstJsonEncoder.cpp:1447-1451` (hand-ported)
use ulua_ast::records::ast_stat_error::AstStatError;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_error(&mut self, node: *mut AstStatError) -> bool {
    unsafe { self.write_ast_stat_error(node) };
    false
  }
}
