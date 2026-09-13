//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ControlFlowGraph.h:49:refinement_id`
//! Source: `Analysis/include/Luau/ControlFlowGraph.h:49` (hand-ported)
// C++ `using RefinementId = NotNull<Refinement>` over ControlFlowGraph.h's
// OWN Refinement variant (previously mis-aliased to Refinement.h's unrelated
// RefinementId).
use crate::type_aliases::refinement_control_flow_graph::Refinement;
pub type RefinementId = *mut Refinement;
