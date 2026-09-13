use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B, luau_insn_d::LUAU_INSN_D,
    luau_insn_op::LUAU_INSN_OP, luau_unreachable::LUAU_UNREACHABLE,
  },
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd, macros::codegen_assert::CODEGEN_ASSERT, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_new_closure(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let pc_val = unsafe { *pc };
  let d = LUAU_INSN_D(pc_val) as u32;
  CODEGEN_ASSERT!(d < unsafe { (*build.function.proto).sizep as u32 });

  let ra = LUAU_INSN_A(pc_val) as u8;
  let pv = unsafe { *(*build.function.proto).p.add(d as usize) };

  let savedpc_op = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_op);

  let env = build.inst_ir_cmd(IrCmd::LoadEnv);
  let nups_op = build.const_uint(unsafe { (*pv).nups as u32 });
  let d_op = build.const_uint(d);
  let ncl = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::NEWCLOSURE, nups_op, env, d_op);

  let ra_op = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, ra_op, ncl);
  let function_tag = build.const_tag(LuaType::Function as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_op, function_tag);

  let nups = unsafe { (*pv).nups as i32 };
  for ui in 0..nups {
    let uinsn = unsafe { *pc.add(ui as usize + 1) };
    CODEGEN_ASSERT!(LUAU_INSN_OP(uinsn) == LuauOpcode::LOP_CAPTURE as u32);

    let capture_type = LUAU_INSN_A(uinsn) as u8;
    match capture_type {
      x if x == LuauCaptureType::LCT_VAL as u8 => {
        let reg_src = build.vm_reg(LUAU_INSN_B(uinsn) as u8);
        let src = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_src);
        let upvalue = build.vm_upvalue(ui as u8);
        let dst = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, ncl, upvalue);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, dst, src);
      }
      x if x == LuauCaptureType::LCT_REF as u8 => {
        let reg_src = build.vm_reg(LUAU_INSN_B(uinsn) as u8);
        let src = build.inst_ir_cmd_ir_op(IrCmd::FINDUPVAL, reg_src);
        let upvalue = build.vm_upvalue(ui as u8);
        let dst = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, ncl, upvalue);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, dst, src);
        let upval_tag = build.const_tag(LuaType::Upval as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, dst, upval_tag);
      }
      x if x == LuauCaptureType::LCT_UPVAL as u8 => {
        let undef = build.undef();
        let src_upvalue = build.vm_upvalue(LUAU_INSN_B(uinsn) as u8);
        let src = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, undef, src_upvalue);
        let dst_upvalue = build.vm_upvalue(ui as u8);
        let dst = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, ncl, dst_upvalue);
        let load = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, src);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, dst, load);
      }
      _ => {
        CODEGEN_ASSERT!(false, "Unknown upvalue capture type");
        LUAU_UNREACHABLE!();
      }
    }
  }

  build.inst_ir_cmd(IrCmd::CheckGc);
}
