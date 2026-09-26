use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::Handle, proposition_control_flow_graph::Proposition,
    refinement_arena_control_flow_graph::RefinementArena,
  },
  type_aliases::{
    def_id_control_flow_graph::DefId, refinement_control_flow_graph::Refinement,
    refinement_id_control_flow_graph::RefinementId,
  },
};

impl RefinementArena {
  pub fn proposition_def_id_bool(&mut self, def: DefId, sense: bool) -> RefinementId {
    let refinement_ptr = self
      .allocator
      .allocate(Refinement::Proposition(Proposition {
        ptr: def,
        r#type: None,
        is_typeof: false,
        sense,
      }));

    LUAU_ASSERT!(!refinement_ptr.is_null());
    // `allocate` 恒返回非空稳定槽位（见上方断言与 append_block 的 bad_alloc
    // panic），`from_ptr` 包装即兑现 `Handle` 的类型级非空契约。
    Handle::from_ptr(refinement_ptr)
  }
}
