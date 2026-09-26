//! Source: `Analysis/src/AstJsonEncoder.cpp:804-820` (hand-ported)
use ulua_ast::records::ast_stat_for::AstStatFor;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// 可缺省的 `step` 是裸指针槽位，仅以 `is_null()` 判空后作为指针下传桥接。
  pub fn write_ast_stat_for(&mut self, node: &AstStatFor) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstStatFor", |e| {
      e.write("var", &node.var);
      e.write("from", &node.from);
      e.write("to", &node.to);
      if !node.step.is_null() {
        e.write("step", &node.step);
      }
      e.write("body", &node.body);
      e.write("hasDo", &node.has_do);
    });
  }
}
