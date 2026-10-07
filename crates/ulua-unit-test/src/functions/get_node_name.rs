use alloc::string::{String, ToString};

use ulua_analysis::type_aliases::module_name_type::ModuleName;

/// cpp `getNodeName(const TestRequireNode*)`（`Fixture.cpp:48-58`）：模块名的最后
/// 一段即节点显示名；无斜杠时整名即段名。
pub fn get_node_name(module_name: &ModuleName) -> String {
  match module_name.rfind('/') {
    // 借用原串切片，仅末段需要 owned
    Some(last_slash_pos) => module_name[(last_slash_pos + 1)..].to_string(),
    None => module_name.to_string(),
  }
}
