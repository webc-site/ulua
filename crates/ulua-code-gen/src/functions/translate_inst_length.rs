use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    check_table_tag_guard::check_table_tag_guard,
    is_userdata_bytecode_type::is_userdata_bytecode_type,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_length(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let bc_types = build.function.get_bytecode_types_at(pcpos);

  // Safety: pc 指向 proto.code 内一条 LEN 指令主字(译码器保证在 sizecode 界内、Instruction=u32 对齐);
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突, 取代原先对 *pc 的两次重复读取。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;

  if is_userdata_bytecode_type(bc_types.a) {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, reg_ra, reg_rb);
    return;
  }

  let fallback = build.fallback_block(pcpos as u32);

  let vm_reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_rb);
  let a_is_table = bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
  check_table_tag_guard(build, tb, a_is_table, pcpos, fallback);

  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_rb);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);

  let va = build.inst_ir_cmd_ir_op(IrCmd::TableLen, vb);
  let vai = build.inst_ir_cmd_ir_op(IrCmd::IntToNum, va);

  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, vai);
  let reg_ra = build.vm_reg(ra);
  build.store_tag(reg_ra, LuaType::Number as u8);

  let next = build.block_at_inst((pcpos + 1) as u32);
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, reg_ra, reg_rb);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
