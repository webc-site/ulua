use alloc::string::String;

use crate::records::test_require_node::TestRequireNode;

pub fn get_node_name(node: &TestRequireNode) -> String {
  // 借用原字符串切片，仅无斜杠时才 clone 整串
  if let Some(last_slash_pos) = node.module_name.rfind('/') {
    node.module_name[(last_slash_pos + 1)..].to_string()
  } else {
    node.module_name.clone()
  }
}
