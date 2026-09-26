use core::mem::{offset_of, size_of};

use ulua_code_gen::records::{ir_builder::IrBuilder, ir_op::IrOp};

use crate::common::{
  functions::check_userdata_tag::check_userdata_tag, records::userdata_tags::K_TAG_VERTEX,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vertex {
  pub pos: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
}

impl Vertex {
  /// tag 取自 `userdata_tags`（对应 cpp `ConformanceIrHooks.h:16` 的 `kTagVertex`），
  /// 与 C API 路径共用同一来源，避免上游调整时两条路径静默漂移。
  pub const TAG: i32 = K_TAG_VERTEX as i32;

  pub const OFFSET_POS: usize = offset_of!(Self, pos);
  pub const OFFSET_POS_X: usize = Self::OFFSET_POS;
  pub const OFFSET_POS_Y: usize = Self::OFFSET_POS + size_of::<f32>();
  pub const OFFSET_POS_Z: usize = Self::OFFSET_POS + 2 * size_of::<f32>();

  pub const OFFSET_NORMAL: usize = offset_of!(Self, normal);
  pub const OFFSET_NORMAL_X: usize = Self::OFFSET_NORMAL;
  pub const OFFSET_NORMAL_Y: usize = Self::OFFSET_NORMAL + size_of::<f32>();
  pub const OFFSET_NORMAL_Z: usize = Self::OFFSET_NORMAL + 2 * size_of::<f32>();

  pub const OFFSET_UV: usize = offset_of!(Self, uv);
  pub const OFFSET_UV_X: usize = Self::OFFSET_UV;
  pub const OFFSET_UV_Y: usize = Self::OFFSET_UV + size_of::<f32>();

  #[inline]
  pub fn check_tag(build: &mut IrBuilder, udata: IrOp, pcpos: i32) {
    check_userdata_tag(build, udata, pcpos, Self::TAG);
  }
}
