//! Source: `Analysis/src/AstJsonEncoder.cpp:1417-1421` (hand-ported)
use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
    unsafe { self.write_ast_stat_local_function(node) };
    false
  }
}
