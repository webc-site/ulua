use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  macros::{
    luau_assert::LUAU_UNREACHABLE, luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b,
    luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op,
  },
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::proto_view::{child_proto, with_proto},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub(crate) fn translate_inst_new_closure(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // pc 指向 proto.code 内一条 NEWCLOSURE 指令主字(译码器保证在 sizecode 界内、u32 对齐),
  // 只读取出该字供 A/D 域提取(子原型索引), 纯读无别名冲突。
  let pc_val = code[pcpos as usize];
  let d = luau_insn_d(pc_val) as u32;
  // Proto 结构体字段直读收口进 `with_proto` 门面(§2): sizep 供下面界内校验。
  let sizep = with_proto(build.function.proto, |proto| proto.sizep as u32)
    .expect("translate_inst_new_closure: proto 构造期接线非空(codegen 契约)");
  CODEGEN_ASSERT!(d < sizep);

  let ra = luau_insn_a(pc_val) as u8;
  // 子原型指针: d < sizep 断言保证 `p[d]` 读取界内(与切片索引等价的只读, 收口进门面)。
  let pv = child_proto(build.function.proto, d)
    .expect("translate_inst_new_closure: proto 构造期接线非空(codegen 契约)");

  let savedpc_op = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_op);

  let env = build.inst_ir_cmd(IrCmd::LoadEnv);
  let nups = with_proto(pv, |proto| proto.nups as usize)
    .expect("translate_inst_new_closure: 子原型 pv 非空存活(codegen 契约)");
  let nups_op = build.const_uint(nups as u32);
  let d_op = build.const_uint(d);
  let ncl = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::NEWCLOSURE, nups_op, env, d_op);

  let ra_op = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, ra_op, ncl);
  build.store_tag(ra_op, LuaType::Function as u8);

  let capture_start = pcpos as usize + 1;
  let capture_end = capture_start + nups;
  for (ui, &uinsn) in code[capture_start..capture_end].iter().enumerate() {
    let ui = ui as u8;
    CODEGEN_ASSERT!(luau_insn_op(uinsn) == LuauOpcode::LOP_CAPTURE as u32);

    let capture_type = luau_insn_a(uinsn) as u8;
    match capture_type {
      x if x == LuauCaptureType::LCT_VAL as u8 => {
        let reg_src = build.vm_reg(luau_insn_b(uinsn) as u8);
        let src = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_src);
        let upvalue = build.vm_upvalue(ui);
        let dst = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, ncl, upvalue);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, dst, src);
      }
      x if x == LuauCaptureType::LCT_REF as u8 => {
        let reg_src = build.vm_reg(luau_insn_b(uinsn) as u8);
        let src = build.inst_ir_cmd_ir_op(IrCmd::FINDUPVAL, reg_src);
        let upvalue = build.vm_upvalue(ui);
        let dst = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, ncl, upvalue);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, dst, src);
        build.store_tag(dst, LuaType::Upval as u8);
      }
      x if x == LuauCaptureType::LCT_UPVAL as u8 => {
        let undef = build.undef();
        let src_upvalue = build.vm_upvalue(luau_insn_b(uinsn) as u8);
        let src = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetClosureUpvalAddr, undef, src_upvalue);
        let dst_upvalue = build.vm_upvalue(ui);
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
