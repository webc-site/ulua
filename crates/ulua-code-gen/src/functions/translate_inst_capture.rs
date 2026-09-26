use ulua_common::{
  enums::luau_capture_type::LuauCaptureType,
  macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b},
};

use crate::{
  enums::ir_cmd::IrCmd, macros::codegen_assert::CODEGEN_ASSERT, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_capture(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 CAPTURE 指令起始(译码器按指令长度定位, 在 sizecode 界内),
  // Instruction=u32 且 code 按 u32 对齐, 读一次即得该指令字(合并原两处 *pc 读)。
  let insn = code[pcpos as usize];
  let type_ = luau_insn_a(insn) as u8;
  let index = luau_insn_b(insn) as u8;

  match type_ {
    x if x == LuauCaptureType::LCT_VAL as u8 => {
      let reg = build.vm_reg(index as u8);
      let const_uint = build.const_uint(0);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, reg, const_uint);
    }
    x if x == LuauCaptureType::LCT_REF as u8 => {
      let reg = build.vm_reg(index as u8);
      let const_uint = build.const_uint(1);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, reg, const_uint);
    }
    x if x == LuauCaptureType::LCT_UPVAL as u8 => {
      let upvalue = build.vm_upvalue(index as u8);
      let const_uint = build.const_uint(0);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, upvalue, const_uint);
    }
    _ => {
      CODEGEN_ASSERT!(false, "Unknown upvalue capture type");
    }
  }
}
