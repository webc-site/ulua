use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::collect_operands::collect_operands,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, def_registry::def_as_mut, phi::Phi},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  pub fn resolve_captures(&mut self) {
    for (_symbol, capture) in self.captures.iter() {
      let mut operands: Vec<DefId> = Vec::new();
      for &v in &capture.all_versions[capture.version_offset..] {
        collect_operands(v, &mut operands);
      }

      for capture_def in &capture.capture_defs {
        // 独占写视图：capture 的 phi 节点由 phi_vector_def_id 刚造、operands 必空
        // （cpp 同一 LUAU_ASSERT 前提）；写点与共享迭代 `self.captures` 分属不同
        // 分配，注册表独占写契约见 `records::def_registry`。
        let phi = def_as_mut::<Phi>(*capture_def);
        LUAU_ASSERT!(phi.is_some());
        let phi = phi.expect("capture def 恒为 Phi 变体（构建期不变量）");
        LUAU_ASSERT!(phi.operands.is_empty());
        phi.operands = operands.clone();
      }
    }
  }
}
