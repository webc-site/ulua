//! Source: `Analysis/src/AstJsonEncoder.cpp:1037-1053` (hand-ported)
use ulua_ast::records::ast_table_indexer::AstTableIndexer;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `indexer` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_table_indexer(&mut self, indexer: *mut AstTableIndexer) {
    if !indexer.is_null() {
      let i = unsafe { &*indexer };
      self.write_raw_string_view("{");
      let c = self.push_comma();
      self.write("location", &i.location);
      self.write("index_type", &i.index_type);
      self.write("result_type", &i.result_type);
      self.pop_comma(c);
      self.write_raw_string_view("}");
    } else {
      self.write_raw_string_view("null");
    }
  }
}
