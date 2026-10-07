use alloc::{string::String, vec};

use ulua_analysis::{
  records::{require_alias::RequireAlias, require_node::RequireNode},
  type_aliases::module_name_type::ModuleName,
};

use crate::functions::get_node_name::get_node_name;

/// cpp `TestRequireNode` 的构造点（`Fixture.h:58-74` + `Fixture.cpp:60-70,142-150`）。
///
/// `RequireNode` 在 Rust 侧已具体成纯数据（见 ulua-analysis
/// `records/require_node.rs` 的 DELIBERATE DEVIATION 注），测试替身不再是一个
/// trait 实现类型，只剩这份节点缺省数据：path component 取模块名末段、label 同
/// 名（cpp `getLabel` 即 `getNodeName`）、无 tags、固定挂一个 `defaultalias`、
/// 允许相对路径（cpp 基类缺省）。
pub fn test_require_node(module_name: &ModuleName) -> RequireNode {
  let path_component = get_node_name(module_name);

  RequireNode {
    aliases: vec![RequireAlias::require_alias_string(String::from(
      "defaultalias",
    ))],
    ..RequireNode::new(module_name.clone(), path_component)
  }
}
