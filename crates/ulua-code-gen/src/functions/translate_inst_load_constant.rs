use ulua_common::FFlag;
use ulua_vm::{enums::lua_type::LuaType, type_aliases::t_value::TValue};

use crate::{enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder};

pub fn translate_inst_load_constant(build: &mut IrBuilder, ra: i32, k: i32) {
  let proto = unsafe { &(*build.function.proto) };
  let protok = unsafe { *proto.k.add(k as usize) } as TValue;

  if protok.tt == LuaType::Nil as i32 {
    let ra_reg = build.vm_reg(ra as u8);
    let tag = build.const_tag(LuaType::Nil as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  } else if protok.tt == LuaType::Boolean as i32 {
    let ra_reg = build.vm_reg(ra as u8);
    let value = build.const_int(unsafe { protok.value.b });
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_reg, value);
    let tag = build.const_tag(LuaType::Boolean as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  } else if FFlag::LuauCodegenInteger2.get() && protok.tt == LuaType::Integer as i32 {
    let ra_reg = build.vm_reg(ra as u8);
    let value = build.const_int_64(unsafe { protok.value.l });
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, ra_reg, value);
    let tag = build.const_tag(LuaType::Integer as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  } else if protok.tt == LuaType::Number as i32 {
    let ra_reg = build.vm_reg(ra as u8);
    let value = build.const_double(unsafe { protok.value.n });
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);
    let tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  } else {
    let const_op = build.vm_const(k as u32);
    let offset = build.const_int(0);
    let tag = build.const_tag(protok.tt as u8);
    let load = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LoadTvalue, const_op, offset, tag);
    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, load);
  }
}
