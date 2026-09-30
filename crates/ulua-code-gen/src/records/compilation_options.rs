use alloc::{string::String, vec::Vec};

use crate::records::host_ir_hooks::HostIrHooks;

#[derive(Debug, Clone, Default)]
pub struct CompilationOptions {
  pub flags: u32,
  /// 全部 hook 字段为 Option<fn>，None 的 niche 即全零位，与 zeroed 位等价
  pub hooks: HostIrHooks,

  /// userdata 类型名（这些类型可能有定制 lowering）；空表即 cpp 的 null 数组
  pub userdata_types: Vec<String>,

  pub record_counters: bool,

  /// 为 true 时在 block 之间插入随机 NOP sled，
  /// 使函数内 gadget 偏移不可预测。
  pub nop_padding: bool,
}
