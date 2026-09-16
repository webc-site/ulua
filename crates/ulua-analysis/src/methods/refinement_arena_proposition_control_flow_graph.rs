use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    proposition_control_flow_graph::Proposition,
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
    refinement_ptr
  }
}
