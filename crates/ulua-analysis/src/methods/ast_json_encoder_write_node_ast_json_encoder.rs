use ulua_ast::records::{ast_node::AstNode, location::Location};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// C++ `writeNode(AstNode* node)` 的安全化端口：帧头唯一消费的字段是节点的
  /// `location`，故以共享引用进入即消除裸指针解引用契约；具体节点调用方经
  /// `base` 链直接以 `&node.location` 提供，无需再上转为 `&AstNode`。
  pub fn write_node_ast_node(&mut self, node: &AstNode) {
    self.write("location", &node.location);
  }

  // C++ template writeNode(AstNode*, string_view, F&& f). The [&] lambda
  // becomes FnOnce(&mut Self) so the body can keep writing through the
  // encoder while the wrapper holds the node frame open.
  pub(crate) fn write_node_ast_node_string_view_f<F: FnOnce(&mut Self)>(
    &mut self,
    location: &Location,
    name: &str,
    f: F,
  ) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view(name);
    self.write("location", location);
    f(self);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
