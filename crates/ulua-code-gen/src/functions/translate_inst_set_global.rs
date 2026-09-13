use ulua_common::macros::luau_insn_a::LUAU_INSN_A;
use ulua_vm::type_aliases::lua_node::LuaNode;

use crate::{
  enums::ir_cmd::IrCmd,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_set_global(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as u8;
  let aux = unsafe { *pc.add(1) };

  let fallback = build.fallback_block(pcpos as u32);

  let env = build.inst_ir_cmd(IrCmd::LoadEnv);
  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_slot_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, env, pcpos_op, aux_op);

  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_slot_el, aux_op, fallback);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, env, fallback);

  let reg_ra = build.vm_reg(ra);
  let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
  let offset_val = build.const_int(core::mem::offset_of!(LuaNode, val) as i32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, addr_slot_el, tva, offset_val);

  let undef = build.undef();
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, env, reg_ra, undef);

  let next = build.block_at_inst((pcpos + 2) as u32);
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let pcpos_op = build.const_uint(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackSetglobal, pcpos_op, reg_ra, aux_op);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
