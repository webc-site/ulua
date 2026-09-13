//! Source: `Analysis/src/AstJsonEncoder.cpp:689-712` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock};

use crate::{
  methods::ast_json_encoder_write_primitives::WriteJson, records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_block(&mut self, node: *mut AstStatBlock) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatBlock", |e| {
      e.write_raw_string_view(",\"hasEnd\":");
      n.has_end.write_json(e);
      e.write_raw_string_view(",\"body\":[");
      let mut comma = false;
      for stat in n.body.iter() {
        if comma {
          e.write_raw_string_view(",");
        } else {
          comma = true;
        }
        (*stat).write_json(e);
      }
      e.write_raw_string_view("]");
    });
  }
}
