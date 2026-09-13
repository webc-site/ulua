use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{op_a::op_a, op_b::op_b},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn get_loop_step_k(build: &mut IrBuilder, ra: i32) -> IrOp {
  let active_block_idx = build.active_block_idx as usize;
  let active = &build.function.blocks[active_block_idx];

  if active.start + 2 <= build.function.instructions.len() as u32 {
    let instructions_size = build.function.instructions.len();
    let mut sv = build.function.instructions[instructions_size - 2].clone();
    let mut st = build.function.instructions[instructions_size - 1].clone();

    // We currently expect to match IR generated from LOADN/LOADK so we match a particular sequence of opcodes
    // In the future this can be extended to cover opposite STORE order as well as STORE_SPLIT_TVALUE
    if sv.cmd == IrCmd::StoreDouble
      && op_a(&mut sv).kind() == IrOpKind::VmReg
      && op_a(&mut sv).index() == (ra + 1) as u32
      && op_b(sv.clone()).kind() == IrOpKind::Constant
      && st.cmd == IrCmd::StoreTag
      && op_a(&mut st).kind() == IrOpKind::VmReg
      && op_a(&mut st).index() == (ra + 1) as u32
      && build.function.tag_op(op_b(st.clone())) == LuaType::Number as u8
    {
      return op_b(sv);
    }
  }

  build.undef()
}
