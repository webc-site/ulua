use core::mem::transmute;

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::LUAU_INSN_OP};

use crate::{
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fast_call::is_fast_call,
  },
  records::{bytecode_block::BytecodeBlock, ir_function::IrFunction},
};

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

pub fn build_bytecode_blocks(function: &mut IrFunction, jump_targets: &[u8]) {
  CODEGEN_ASSERT!(!function.proto.is_null());

  let proto = unsafe { &*function.proto };
  let bc_blocks = &mut function.bc_blocks;

  // Using the same jump targets, create VM bytecode basic blocks
  bc_blocks.push(BytecodeBlock {
    startpc: 0,
    finishpc: -1,
  });

  let mut previ = 0;
  let mut i = 0;

  while i < proto.sizecode {
    let pc_val = unsafe { *proto.code.add(i as usize) };
    let op_val = LUAU_INSN_OP(pc_val) as u8;
    let op: LuauOpcode = unsafe { transmute(op_val) };

    let nexti = i + get_op_length(op);

    // If instruction is a jump target, begin new block starting from it
    if i != 0 && jump_targets[i as usize] != 0 {
      if let Some(last) = bc_blocks.last_mut() {
        last.finishpc = previ;
      }
      bc_blocks.push(BytecodeBlock {
        startpc: i,
        finishpc: -1,
      });
    }

    let target = get_jump_target(pc_val, i as u32);

    // Implicit fallthrough terminate the block and might start a new one
    if target >= 0 && !is_fast_call(op) {
      if let Some(last) = bc_blocks.last_mut() {
        last.finishpc = i;
      }

      // Start a new block if there was no explicit jump for the fallthrough
      if jump_targets[nexti as usize] == 0 {
        bc_blocks.push(BytecodeBlock {
          startpc: nexti,
          finishpc: -1,
        });
      }
    }
    // Returns just terminate the block
    else if op == LuauOpcode::LOP_RETURN
      && let Some(last) = bc_blocks.last_mut()
    {
      last.finishpc = i;
    }

    previ = i;
    i = nexti;
    CODEGEN_ASSERT!(i <= proto.sizecode);
  }
}
