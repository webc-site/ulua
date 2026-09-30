//! Source: `Analysis/include/Luau/ControlFlowGraph.h:34` (hand-ported)
use crate::{
  macros::variant_enum,
  records::{assign::Assign, declare::Declare, join::Join, refine::Refine},
};

variant_enum! {
  #[derive(Debug, Clone)]
  pub enum Instruction {
    Declare => Declare,
    Assign => Assign,
    Join => Join,
    Refine => Refine,
  }

  /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
  pub trait InstructionMember;
}
