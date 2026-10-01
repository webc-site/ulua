use alloc::string::String;

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

pub fn refinement_arena_type_proposition(
  arena: &mut RefinementArena,
  def: DefId,
  r#type: Option<String>,
  is_typeof: bool,
  sense: bool,
) -> RefinementId {
  // `allocate` 恒返回非空稳定槽位，`from_ptr` 兑现 Handle 的非空契约。
  Handle::from_ptr(
    arena
      .allocator
      .allocate(Refinement::Proposition(Proposition {
        ptr: def,
        r#type,
        is_typeof,
        sense,
      })),
  )
}
