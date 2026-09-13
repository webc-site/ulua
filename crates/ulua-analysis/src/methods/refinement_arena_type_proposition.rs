use alloc::string::String;

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

pub fn refinement_arena_type_proposition(
  arena: &mut RefinementArena,
  def: DefId,
  r#type: Option<String>,
  is_typeof: bool,
  sense: bool,
) -> RefinementId {
  arena
    .allocator
    .allocate(Refinement::Proposition(Proposition {
      ptr: def,
      r#type,
      is_typeof,
      sense,
    }))
}
