//! Source: `Analysis/include/Luau/ControlFlowGraph.h:233` (hand-ported)
//! C++ `struct CFGAllocator`.
use crate::{
  records::{
    block::Block, refinement_arena_control_flow_graph::RefinementArena,
    typed_allocator::TypedAllocator,
  },
  type_aliases::{definition::Definition, instruction::Instruction},
};

#[derive(Debug)]
pub struct CfgAllocator {
  // public:
  pub refinement_arena: RefinementArena,
  // private:
  pub(crate) block: TypedAllocator<Block>,
  pub(crate) instructions: TypedAllocator<Instruction>,
  pub(crate) defs: TypedAllocator<Definition>,
  pub(crate) frozen: bool,
}

impl Default for CfgAllocator {
  fn default() -> Self {
    Self {
      // `RefinementArena` holds a single `TypedAllocator<Refinement>`
      // (default-constructed). Built via struct literal (field is
      // `pub(crate)`, same crate) to avoid a Default impl on the arena.
      refinement_arena: RefinementArena {
        allocator: TypedAllocator::default(),
      },
      block: TypedAllocator::default(),
      instructions: TypedAllocator::default(),
      defs: TypedAllocator::default(),
      // C++ `bool frozen = false;`
      frozen: false,
    }
  }
}

unsafe impl Send for CfgAllocator {}
unsafe impl Sync for CfgAllocator {}
