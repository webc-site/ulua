use core::mem::offset_of;

use ulua_common::macros::luau_insn_a::luau_insn_a;
use ulua_vm::records::lua_node::LuaNode;

use crate::{
  enums::ir_cmd::IrCmd,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_builder::Instruction,
};

/// 翻译 GETGLOBAL 指令。
pub fn translate_inst_get_global(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_global_access(build, code, pcpos, true);
}

/// GETGLOBAL / SETGLOBAL 共用主体：`is_get` 取 true（读）/ false（写）。
/// 严格保序：CheckReadonly 与 BarrierTableForward 仅 SET（写）侧发射。
pub fn translate_global_access(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  is_get: bool,
) {
  // Safety: pc 指向 proto.code 内一条 GETGLOBAL/SETGLOBAL 指令主字(译码器按 op 长度
  // 定位, 在 sizecode 界内, u32 对齐), 只读取出该字供 A 域提取, 纯读无别名冲突。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  // Safety: GETGLOBAL/SETGLOBAL 均为 2 字指令, pc.add(1) 为其 AUX 字(常量/槽索引),
  // 译码器已确认落在 code/sizecode 界内且 u32 对齐, 读取合法。
  let aux = code[pcpos as usize + 1];

  let fallback = build.fallback_block(pcpos as u32);

  let env = build.inst_ir_cmd(IrCmd::LoadEnv);
  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_slot_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, env, pcpos_op, aux_op);

  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_slot_el, aux_op, fallback);

  if is_get {
    let offset_val = build.const_int(offset_of!(LuaNode, val) as i32);
    let tvn = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_slot_el, offset_val);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, tvn);
  } else {
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, env, fallback);

    let reg_ra = build.vm_reg(ra);
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
    let offset_val = build.const_int(offset_of!(LuaNode, val) as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, addr_slot_el, tva, offset_val);

    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, env, reg_ra, undef);
  }

  let next = build.block_at_inst((pcpos + 2) as u32);
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let pcpos_op = build.const_uint(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(
    if is_get {
      IrCmd::FallbackGetglobal
    } else {
      IrCmd::FallbackSetglobal
    },
    pcpos_op,
    reg_ra,
    aux_op,
  );
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
