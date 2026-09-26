use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{op_a_ref::op_a_ref, op_b_ref::op_b_ref},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn get_loop_step_k(build: &mut IrBuilder, ra: i32) -> IrOp {
  let active_block_idx = build.active_block_idx as usize;
  let active = &build.function.blocks[active_block_idx];

  if active.start + 2 <= build.function.instructions.len() as u32 {
    let instructions_size = build.function.instructions.len();
    // 纯只读匹配，无需克隆指令
    let sv = &build.function.instructions[instructions_size - 2];
    let st = &build.function.instructions[instructions_size - 1];

    // 目前只需匹配 LOADN/LOADK 生成的 IR，因此匹配特定 opcode 序列
    // 未来可扩展到相反的 STORE 顺序以及 STORE_SPLIT_TVALUE
    if sv.cmd == IrCmd::StoreDouble
      && op_a_ref(sv).kind() == IrOpKind::VmReg
      && op_a_ref(sv).index() == (ra + 1) as u32
      && op_b_ref(sv).kind() == IrOpKind::Constant
      && st.cmd == IrCmd::StoreTag
      && op_a_ref(st).kind() == IrOpKind::VmReg
      && op_a_ref(st).index() == (ra + 1) as u32
      && build.function.tag_op(op_b_ref(st)) == LuaType::Number as u8
    {
      return op_b_ref(sv);
    }
  }

  build.undef()
}
