use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{
    host_ir_hooks::HostIrHooks,
    ir_builder::{ConstantKey, IrBuilder},
  },
};

impl IrBuilder {
  pub fn ir_builder_ir_builder(host_hooks: &HostIrHooks) -> Self {
    Self {
      host_hooks: host_hooks as *const HostIrHooks,
      in_terminated_block: false,
      interrupt_requested: false,
      active_fastcall_fallback: false,
      fastcall_fallback_return: Default::default(),
      cmd_skip_target: -1,
      function: Default::default(),
      active_block_idx: !0u32,
      inst_index_to_block: Vec::new(),
      numeric_loop_stack: Vec::new(),
      constant_map: DenseHashMap::new(ConstantKey {
        kind: IrConstKind::Tag,
        value: !0u64,
      }),
    }
  }
}
