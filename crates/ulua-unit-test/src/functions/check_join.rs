use ulua_analysis::{
  records::{join::Join, sym_def_registry::resolve_sym_def},
  type_aliases::def_id_control_flow_graph::DefId,
};

/// 句柄 → 版本名：j.definition/j.operands 由 fixture 构建时经注册表发放
/// （NotNull 语义非空），解析未命中即契约被破坏，panic 比 cpp 悬垂解引用保守。
fn versioned_name(def: DefId) -> String {
  resolve_sym_def(def)
    .expect("DefId 非空（NotNull 语义）")
    .versioned_name()
}

/// 断言 join 节点的定义名与操作数定义名逐一匹配。
pub fn check_join(j: &Join, def: &str, operands: &[&str]) {
  assert_eq!(versioned_name(j.definition), def);
  assert_eq!(j.operands.len(), operands.len());

  for (i, operand_def) in operands.iter().enumerate() {
    assert_eq!(versioned_name(j.operands[i]), *operand_def);
  }
}
