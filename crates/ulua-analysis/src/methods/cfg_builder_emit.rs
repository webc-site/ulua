//! Source: `Analysis/include/Luau/ControlFlowGraph.h:306-312` (hand-ported)
//! C++ `template<typename T, typename... Args> NotNull<T> CFGBuilder::emit(Block* block, Args&&... args)`.
/// Bridges the C++ `T{args...}` variadic construction: maps each instruction
/// type `T` and its constructor arg-pack to a built `Instruction` variant.
/// (C++ `instructions.allocate(T{std::forward<Args>(args)...})`.)
use alloc::vec::Vec;

use ulua_ast::records::{ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal};

use crate::{
  records::{
    assign::Assign, block::Block, cfg_builder::CfgBuilder, declare::Declare, join::Join,
    refine::Refine,
  },
  type_aliases::{
    def_id_control_flow_graph::DefId,
    instruction::{Instruction, InstructionMember},
    refinement_control_flow_graph::Refinement,
  },
};
pub trait IntoInstruction<T> {
  fn into_instruction(self) -> Instruction;
}

// emit<Declare>(block, (def, source)) -> Declare(def, source)
impl IntoInstruction<Declare> for (DefId, *mut AstStatLocal) {
  fn into_instruction(self) -> Instruction {
    Instruction::Declare(Declare::new(self.0, self.1))
  }
}

// emit<Assign>(block, (def, source)) -> Assign(def, source)
impl IntoInstruction<Assign> for (DefId, *mut AstStatAssign) {
  fn into_instruction(self) -> Instruction {
    Instruction::Assign(Assign::new(self.0, self.1))
  }
}

// emit<Join>(block, def) -> Join(definition)
impl IntoInstruction<Join> for DefId {
  fn into_instruction(self) -> Instruction {
    // C++ `explicit Join(DefId definition)` — operands start empty.
    Instruction::Join(Join {
      definition: self,
      operands: Vec::new(),
    })
  }
}

// emit<Refine>(block, (definition, prop)) -> Refine(definition, prop)
impl IntoInstruction<Refine> for (DefId, *const Refinement) {
  fn into_instruction(self) -> Instruction {
    Instruction::Refine(Refine {
      definition: self.0,
      prop: self.1,
    })
  }
}

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `template<typename T, typename... Args> NotNull<T> emit(Block* block, Args&&...)`.
  /// `NotNull<T>` -> `*mut T`.
  pub(crate) fn emit<T, Args>(&mut self, block: *mut Block, args: Args) -> *mut T
  where
    Args: IntoInstruction<T>,
    T: InstructionMember,
  {
    // C++:
    //   InstrId inst = allocator->newInstruction<T>(std::forward<Args>(args)...);
    //   block->instructions.emplace_back(inst);
    //   return NotNull{inst->template get_if<T>()};
    let allocator = unsafe { &mut *self.allocator };
    let inst = allocator.new_instruction(args.into_instruction());
    unsafe {
      (*block).instructions.push(inst);
      <T as InstructionMember>::get_if_mut(&mut *inst).unwrap() as *mut T
    }
  }
}
