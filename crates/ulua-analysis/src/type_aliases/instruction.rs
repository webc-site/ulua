//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ControlFlowGraph.h:34:instruction`
//! Source: `Analysis/include/Luau/ControlFlowGraph.h:34` (hand-ported)
use crate::records::{assign::Assign, declare::Declare, join::Join, refine::Refine};

#[derive(Debug, Clone)]
pub enum Instruction {
  Declare(Declare),
  Assign(Assign),
  Join(Join),
  Refine(Refine),
}

impl Instruction {
  /// C++ `v.index()` — the member's position in the Variant<...> list.
  pub fn index(&self) -> i32 {
    match self {
      Instruction::Declare(_) => 0,
      Instruction::Assign(_) => 1,
      Instruction::Join(_) => 2,
      Instruction::Refine(_) => 3,
    }
  }
}

/// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
pub trait InstructionMember: Sized {
  fn get_if(v: &Instruction) -> Option<&Self>;
  fn get_if_mut(v: &mut Instruction) -> Option<&mut Self>;
}

impl InstructionMember for Declare {
  fn get_if(v: &Instruction) -> Option<&Self> {
    match v {
      Instruction::Declare(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Instruction) -> Option<&mut Self> {
    match v {
      Instruction::Declare(x) => Some(x),
      _ => None,
    }
  }
}

impl InstructionMember for Assign {
  fn get_if(v: &Instruction) -> Option<&Self> {
    match v {
      Instruction::Assign(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Instruction) -> Option<&mut Self> {
    match v {
      Instruction::Assign(x) => Some(x),
      _ => None,
    }
  }
}

impl InstructionMember for Join {
  fn get_if(v: &Instruction) -> Option<&Self> {
    match v {
      Instruction::Join(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Instruction) -> Option<&mut Self> {
    match v {
      Instruction::Join(x) => Some(x),
      _ => None,
    }
  }
}

impl InstructionMember for Refine {
  fn get_if(v: &Instruction) -> Option<&Self> {
    match v {
      Instruction::Refine(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Instruction) -> Option<&mut Self> {
    match v {
      Instruction::Refine(x) => Some(x),
      _ => None,
    }
  }
}
