use ulua_common::{
  fflag,
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_c, luau_insn_d},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::proto_view::{
    constant_boolean, constant_integer, constant_number, with_constant_value,
  },
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// 翻译 LOP_LOADNIL 指令。
pub fn translate_inst_load_nil(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let ra = luau_insn_a(code[pcpos as usize]) as u8;
  let vm_reg = build.vm_reg(ra);
  build.store_tag(vm_reg, LuaType::Nil as u8);
}

/// 翻译 LOP_LOADB 指令。
pub fn translate_inst_load_b(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let pc_val = code[pcpos as usize];
  let ra = luau_insn_a(pc_val) as u8;
  let b = luau_insn_b(pc_val);
  let c = luau_insn_c(pc_val);

  let ra_op = build.vm_reg(ra);
  let b_op = build.const_int(b as i32);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_op, b_op);

  let tag_op = build.const_tag(LuaType::Boolean as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_op, tag_op);

  if c != 0 {
    let target = pcpos + 1 + (c as i32);
    let block = build.block_at_inst(target as u32);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
  }
}

/// 翻译 LOP_LOADN 指令。
pub fn translate_inst_load_n(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let vm_reg = build.vm_reg(ra);
  let value = build.const_double(luau_insn_d(insn) as f64);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, vm_reg, value);
  build.store_tag(vm_reg, LuaType::Number as u8);
}

/// 翻译 LOP_LOADK 指令。
pub fn translate_inst_load_k(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let insn = code[pcpos as usize];
  translate_inst_load_constant(build, luau_insn_a(insn) as i32, luau_insn_d(insn));
}

/// 翻译 LOP_LOADKX 指令。
pub fn translate_inst_load_kx(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let ra = luau_insn_a(code[pcpos as usize]) as i32;
  let kx = code[pcpos as usize + 1] as i32;
  translate_inst_load_constant(build, ra, kx);
}

/// 常量池加载通用逻辑。
pub fn translate_inst_load_constant(build: &mut IrBuilder, ra: i32, k: i32) {
  let proto = build.function.proto;
  let k = k as u32;
  with_constant_value(proto, k, |tv| {
    let tt = tv.tt;
    if tt == LuaType::Nil as i32 {
      let ra_reg = build.vm_reg(ra as u8);
      build.store_tag(ra_reg, LuaType::Nil as u8);
    } else if tt == LuaType::Boolean as i32 {
      let ra_reg = build.vm_reg(ra as u8);
      let value = build.const_int(constant_boolean(tv));
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_reg, value);
      build.store_tag(ra_reg, LuaType::Boolean as u8);
    } else if fflag::LuauCodegenInteger3.get() && tt == LuaType::Integer as i32 {
      let ra_reg = build.vm_reg(ra as u8);
      let value = build.const_int_64(constant_integer(tv));
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, ra_reg, value);
      build.store_tag(ra_reg, LuaType::Integer as u8);
    } else if tt == LuaType::Number as i32 {
      let ra_reg = build.vm_reg(ra as u8);
      let value = build.const_double(constant_number(tv));
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);
      build.store_tag(ra_reg, LuaType::Number as u8);
    } else {
      let const_op = build.vm_const(k);
      let offset = build.const_int(0);
      let tag = build.const_tag(tt as u8);
      let load = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, const_op, offset, tag);
      let ra_reg = build.vm_reg(ra as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, load);
    }
  })
  .expect("translate_inst_load_constant: proto/k 构造期接线非空, k<sizek 界内(codegen 契约)");
}
