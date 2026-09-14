use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B,
    luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{gco_2_ts::gco2ts, getstr::getstr},
  records::{g_cheader::GCheader, t_string::tstring},
};

use crate::{
  enums::ir_cmd::IrCmd,
  functions::is_userdata_bytecode_type::is_userdata_bytecode_type,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_translation::Instruction,
};

#[repr(C)]
struct tstringHeader {
  hdr: GCheader,
  _padding1: [u8; 1],
  atom: i16,
  _padding2: [u8; 2],
  next: *mut tstring,
  hash: u32,
  len: u32,
  data: [u8; 1],
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_get_table_ks(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as u8;
  let rb = LUAU_INSN_B(insn) as u8;

  let op = LUAU_INSN_OP(insn);
  let aux = if LuauOpcode::from(op as u8) == LuauOpcode::LOP_GETUDATAKS {
    LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
  } else {
    unsafe { *pc.add(1) }
  } as u32;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);

  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
    let vector_tag = build.const_tag(LuaType::Vector as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, vector_tag, exit);

    let proto = build.function.proto;
    let k = unsafe { (*proto).k };
    let aux_idx = aux as usize;
    let val = unsafe { (*k.add(aux_idx)).value };
    let gc = unsafe { val.gc };
    let ts = unsafe { gco2ts!(gc) as *const _ as *const tstring };
    let field = unsafe { getstr(ts) };
    let len = unsafe { (*(ts as *const tstringHeader)).len };

    if len == 1 {
      let field_first = unsafe { *field } as u8;
      if field_first == b'X' || field_first == b'x' {
        let reg_rb = build.vm_reg(rb);
        let offset = build.const_int(0);
        let value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, reg_rb, offset);
        let value = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, value);
        let reg_ra = build.vm_reg(ra);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, value);
        let reg_ra = build.vm_reg(ra);
        let number_tag = build.const_tag(LuaType::Number as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_ra, number_tag);
        return;
      } else if field_first == b'Y' || field_first == b'y' {
        let reg_rb = build.vm_reg(rb);
        let offset = build.const_int(4);
        let value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, reg_rb, offset);
        let value = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, value);
        let reg_ra = build.vm_reg(ra);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, value);
        let reg_ra = build.vm_reg(ra);
        let number_tag = build.const_tag(LuaType::Number as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_ra, number_tag);
        return;
      } else if field_first == b'Z' || field_first == b'z' {
        let reg_rb = build.vm_reg(rb);
        let offset = build.const_int(8);
        let value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, reg_rb, offset);
        let value = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, value);
        let reg_ra = build.vm_reg(ra);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, value);
        let reg_ra = build.vm_reg(ra);
        let number_tag = build.const_tag(LuaType::Number as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_ra, number_tag);
        return;
      }
    }

    let host_hooks = unsafe { &*build.host_hooks };
    if let Some(vector_access) = host_hooks.vector_access {
      let handled = unsafe {
        vector_access(
          build as *mut IrBuilder,
          field,
          len as usize,
          ra as i32,
          rb as i32,
          pcpos,
        )
      };
      if handled {
        return;
      }
    }

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackGettableks,
      pcpos_op,
      reg_ra,
      reg_rb,
      aux_op,
    );
    return;
  }

  if is_userdata_bytecode_type(bc_types.a) {
    let userdata_tag = build.const_tag(LuaType::UserData as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, userdata_tag, exit);

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackGettableks,
      pcpos_op,
      reg_ra,
      reg_rb,
      aux_op,
    );
    return;
  }

  let fallback = build.fallback_block(pcpos as u32);

  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  let table_tag = build.const_tag(LuaType::Table as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, table_tag, exit_or_fallback);

  let reg_rb = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_slot_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, vb, pcpos_op, aux_op);

  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_slot_el, aux_op, fallback);

  let offsetof_val = build.const_int(0);
  let tvn = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_slot_el, offsetof_val);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, tvn);

  let next = build.block_at_inst((pcpos + 2) as u32);
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let pcpos_op = build.const_uint(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::FallbackGettableks,
    pcpos_op,
    reg_ra,
    reg_rb,
    aux_op,
  );
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
