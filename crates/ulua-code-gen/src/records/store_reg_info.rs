#[derive(Debug, Clone)]
#[repr(C)]
pub struct StoreRegInfo {
  pub tag_inst_idx: u32,
  pub value_inst_idx: u32,
  pub tvalue_inst_idx: u32,

  pub maybe_gco: bool,

  pub ignore_at_exit: bool,

  pub known_tag: u8,
}

impl Default for StoreRegInfo {
  fn default() -> Self {
    Self {
      tag_inst_idx: !0u32,
      value_inst_idx: !0u32,
      tvalue_inst_idx: !0u32,
      maybe_gco: false,
      ignore_at_exit: false,
      known_tag: 0xff,
    }
  }
}
