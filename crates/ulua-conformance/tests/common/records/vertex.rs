use core::mem::size_of;

use ulua_code_gen::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

use crate::common::enums::userdata_kind::UserdataKind;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vertex {
  pub pos: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
}

impl Vertex {
  pub const TAG: i32 = 13;
  pub const USERDATA_INDEX: u8 = UserdataKind::VERTEX;
  pub const SIZE: usize = size_of::<Self>();

  pub const OFFSET_POS: usize = core::mem::offset_of!(Self, pos);
  pub const OFFSET_POS_X: usize = Self::OFFSET_POS;
  pub const OFFSET_POS_Y: usize = Self::OFFSET_POS + size_of::<f32>();
  pub const OFFSET_POS_Z: usize = Self::OFFSET_POS + 2 * size_of::<f32>();

  pub const OFFSET_NORMAL: usize = core::mem::offset_of!(Self, normal);
  pub const OFFSET_NORMAL_X: usize = Self::OFFSET_NORMAL;
  pub const OFFSET_NORMAL_Y: usize = Self::OFFSET_NORMAL + size_of::<f32>();
  pub const OFFSET_NORMAL_Z: usize = Self::OFFSET_NORMAL + 2 * size_of::<f32>();

  pub const OFFSET_UV: usize = core::mem::offset_of!(Self, uv);
  pub const OFFSET_UV_X: usize = Self::OFFSET_UV;
  pub const OFFSET_UV_Y: usize = Self::OFFSET_UV + size_of::<f32>();

  #[inline]
  pub fn check_tag(build: &mut IrBuilder, udata: IrOp, pcpos: i32) {
    let tag = build.const_int(Self::TAG);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckUserdataTag, udata, tag, exit);
  }
}
