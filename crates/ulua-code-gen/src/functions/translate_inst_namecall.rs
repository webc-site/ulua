use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B,
    luau_insn_c::LUAU_INSN_C, luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  macros::{getstr::getstr, tsvalue::tsvalue},
  records::{g_cheader::GCheader, t_string::tstring},
};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  functions::{get_op_length::get_op_length, is_userdata_bytecode_type::is_userdata_bytecode_type},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
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
pub unsafe fn translate_inst_namecall(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) -> bool {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

  let op = LUAU_INSN_OP(unsafe { *pc });
  let aux = if LuauOpcode::from(op as u8) == LuauOpcode::LOP_NAMECALLUDATA {
    LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
  } else {
    unsafe { *pc.add(1) }
  } as u32;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
    let reg_rb = build.vm_reg(rb);
    let exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(reg_rb, LuaType::Vector as u8, exit);

    let vector_namecall = unsafe { (*build.host_hooks).vector_namecall };
    if let Some(vector_namecall) = vector_namecall {
      let call = unsafe { *pc.add(2) };
      let call_op = LuauOpcode::from(LUAU_INSN_OP(call) as u8);
      CODEGEN_ASSERT!(call_op == LuauOpcode::LOP_CALL || call_op == LuauOpcode::LOP_CALLFB);

      let callra = LUAU_INSN_A(call) as i32;
      let nparams = LUAU_INSN_B(call) as i32 - 1;
      let nresults = LUAU_INSN_C(call) as i32 - 1;

      let proto_k = unsafe { (*build.function.proto).k.add(aux as usize) };
      let ts = unsafe { tsvalue!(proto_k) };
      let field = unsafe { getstr(ts) };
      let len = unsafe { (*(ts as *const tstringHeader)).len } as usize;

      let handled = unsafe {
        vector_namecall(
          build as *mut IrBuilder,
          field,
          len,
          callra,
          rb as i32,
          nparams,
          nresults,
          pcpos,
        )
      };
      if handled {
        return true;
      }
    }

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackNamecall,
      pcpos_op,
      reg_ra,
      reg_rb,
      aux_op,
    );
    return false;
  }

  if is_userdata_bytecode_type(bc_types.a) {
    let reg_rb = build.vm_reg(rb);
    let exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(reg_rb, LuaType::UserData as u8, exit);

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackNamecall,
      pcpos_op,
      reg_ra,
      reg_rb,
      aux_op,
    );
    return false;
  }

  let next = build.block_at_inst((pcpos + get_op_length(LuauOpcode::LOP_NAMECALL)) as u32);
  let fallback = build.fallback_block(pcpos as u32);
  let first_fast_path_success = build.block(IrBlockKind::Internal);
  let second_fast_path = build.block(IrBlockKind::Internal);

  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  let reg_rb = build.vm_reg(rb);
  build.load_and_check_tag(reg_rb, LuaType::Table as u8, exit_or_fallback);
  let reg_rb = build.vm_reg(rb);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  CODEGEN_ASSERT!(!build.function.proto.is_null());
  let proto_k = unsafe { (*build.function.proto).k.add(aux as usize) };
  let ts = unsafe { tsvalue!(proto_k) };
  let hash = unsafe { (*(ts as *const tstringHeader)).hash };
  let hash_op = build.const_uint(hash as u32);
  let addr_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddr, table, hash_op);

  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpSlotMatch,
    addr_node_el,
    aux_op,
    first_fast_path_success,
    second_fast_path,
  );

  build.begin_block(first_fast_path_success);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, reg_self, table);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  let table_tag = build.const_tag(LuaType::Table as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_self, table_tag);

  let offset_val = build.const_int(0);
  let node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_node_el, offset_val);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, node_el);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  build.begin_block(second_fast_path);

  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNodeNoNext, addr_node_el, fallback);

  let tm_index = build.const_int(TMS::TmIndex as i32);
  let index_ptr =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::TryCallFastgettm, table, tm_index, fallback);

  build.load_and_check_tag(index_ptr, LuaType::Table as u8, fallback);
  let index = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, index_ptr);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_index_node_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, index, pcpos_op, aux_op);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_index_node_el, aux_op, fallback);

  let reg_rb = build.vm_reg(rb);
  let table2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, reg_self, table2);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  let table_tag = build.const_tag(LuaType::Table as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_self, table_tag);

  let zero = build.const_int(0);
  let index_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_index_node_el, zero);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, index_node_el);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  build.begin_block(fallback);
  let pcpos_op = build.const_uint(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::FallbackNamecall,
    pcpos_op,
    reg_ra,
    reg_rb,
    aux_op,
  );
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  build.begin_block(next);

  false
}
