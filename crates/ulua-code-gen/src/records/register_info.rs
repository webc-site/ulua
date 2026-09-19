use crate::records::ir_op::IrOp;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RegisterInfo {
  pub tag: u8,
  pub value: IrOp,
  pub version: u32,

  /// `LuauCodegenExtraTableOpts` 关闭时走的旧路径：缓存 NewTable 的 array size，
  /// `-1` 表示未知。
  pub known_table_array_size: i32,
}

impl Default for RegisterInfo {
  fn default() -> Self {
    Self {
      tag: 0xff,
      value: IrOp::default(),
      version: 0,
      known_table_array_size: -1,
    }
  }
}
