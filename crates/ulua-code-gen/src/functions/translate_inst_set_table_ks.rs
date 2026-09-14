use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B,
    luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{enums::lua_type::LuaType, type_aliases::lua_node::LuaNode};

use crate::{
  enums::ir_cmd::IrCmd,
  functions::is_userdata_bytecode_type::is_userdata_bytecode_type,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_set_table_ks(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as u8;
  let rb = LUAU_INSN_B(insn) as u8;

  let aux = if LuauOpcode::from(LUAU_INSN_OP(insn) as u8) == LuauOpcode::LOP_SETUDATAKS {
    LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
  } else {
    unsafe { *pc.add(1) }
  };

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  let rb_reg = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, rb_reg);

  if is_userdata_bytecode_type(bc_types.a) {
    let userdata_tag = build.const_tag(LuaType::UserData as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, userdata_tag, exit);

    let pcpos_op = build.const_uint(pcpos as u32);
    let ra_op = build.vm_reg(ra);
    let rb_op = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackSettableks,
      pcpos_op,
      ra_op,
      rb_op,
      aux_op,
    );
    return;
  }

  let fallback = build.fallback_block(pcpos as u32);
  let table_tag = build.const_tag(LuaType::Table as u8);
  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, table_tag, exit_or_fallback);

  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, rb_reg);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_slot_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, vb, pcpos_op, aux_op);

  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_slot_el, aux_op, fallback);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);

  let ra_op = build.vm_reg(ra);
  let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, ra_op);
  let offset = build.const_int(core::mem::offset_of!(LuaNode, val) as i32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, addr_slot_el, tva, offset);

  let undef = build.undef();
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, ra_op, undef);

  let next = build.block_at_inst((pcpos + 2) as u32);
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let pcpos_op = build.const_uint(pcpos as u32);
  let ra_op = build.vm_reg(ra);
  let rb_op = build.vm_reg(rb);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::FallbackSettableks,
    pcpos_op,
    ra_op,
    rb_op,
    aux_op,
  );
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
