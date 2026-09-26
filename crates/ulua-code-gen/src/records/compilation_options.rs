use core::{ffi::c_char, ptr::null};

use crate::records::host_ir_hooks::HostIrHooks;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CompilationOptions {
  pub flags: u32,
  pub hooks: HostIrHooks,

  /// 以 null 结尾的 userdata 类型名数组，这些类型可能有定制 lowering
  pub userdata_types: *const *const c_char,

  pub record_counters: bool,

  /// 为 true 时在 block 之间插入随机 NOP sled，
  /// 使函数内 gadget 偏移不可预测。
  pub nop_padding: bool,
}

impl Default for CompilationOptions {
  fn default() -> Self {
    Self {
      flags: 0,
      // 全部字段为 Option<fn>，None 的 niche 即全零位，与 zeroed 位等价
      hooks: HostIrHooks::default(),
      userdata_types: null(),
      record_counters: false,
      nop_padding: false,
    }
  }
}
