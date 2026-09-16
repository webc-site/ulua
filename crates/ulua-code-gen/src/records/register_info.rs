use crate::records::ir_op::IrOp;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RegisterInfo {
  pub tag: u8,
  pub value: IrOp,
  pub version: u32,

  pub known_not_readonly_deprecated: bool,
  pub known_no_metatable_deprecated: bool,
  pub known_table_array_size_deprecated: i32,
}

impl Default for RegisterInfo {
  fn default() -> Self {
    Self {
      tag: 0xff,
      value: IrOp::default(),
      version: 0,
      known_not_readonly_deprecated: false,
      known_no_metatable_deprecated: false,
      known_table_array_size_deprecated: -1,
    }
  }
}
