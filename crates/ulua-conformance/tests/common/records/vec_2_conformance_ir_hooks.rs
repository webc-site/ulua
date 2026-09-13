use core::mem::size_of;

use ulua_code_gen::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::common::enums::userdata_kind::UserdataKind;
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub const TAG: i32 = 12;
  pub const USERDATA_INDEX: u8 = UserdataKind::VEC2;
  pub const SIZE: usize = size_of::<Self>();
  pub const OFFSET_X: usize = core::mem::offset_of!(Self, x);
  pub const OFFSET_Y: usize = core::mem::offset_of!(Self, y);

  #[inline]
  pub fn check_tag(build: &mut IrBuilder, udata: IrOp, pcpos: i32) {
    let tag = build.const_int(Self::TAG);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckUserdataTag, udata, tag, exit);
  }

  #[inline]
  pub fn read_field(build: &mut IrBuilder, udata: IrOp, offset: usize) -> IrOp {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(LuaType::UserData as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BufferReadf32, udata, offset, tag)
  }

  #[inline]
  pub fn write_field(build: &mut IrBuilder, udata: IrOp, offset: usize, value: IrOp) {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(LuaType::UserData as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritef32, udata, offset, value, tag);
  }

  #[inline]
  pub fn new_userdata(build: &mut IrBuilder) -> IrOp {
    build.inst_ir_cmd(IrCmd::CheckGc);
    let size = build.const_int(Self::SIZE as i32);
    let tag = build.const_int(Self::TAG);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::NewUserdata, size, tag)
  }
}
