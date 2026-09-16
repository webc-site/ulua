use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::get_xmm_register_count::get_xmm_register_count,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_function::IrFunction,
    ir_reg_alloc_x_64::IrRegAllocX64, lowering_stats::LoweringStats,
  },
};

impl IrRegAllocX64 {
  pub fn ir_reg_alloc_x_64_ir_reg_alloc_x_64(
    build: &mut AssemblyBuilderX64,
    function: &mut IrFunction,
    stats: *mut LoweringStats,
  ) -> Self {
    let usable_xmm_reg_count = get_xmm_register_count(build.abi);

    Self {
      build: build as *mut AssemblyBuilderX64,
      function: function as *mut IrFunction,
      stats,
      curr_inst_idx: !0u32,
      free_gpr_map: [true; 16],
      gpr_inst_users: [u32::MAX; 16],
      free_xmm_map: [true; 16],
      xmm_inst_users: [u32::MAX; 16],
      usable_xmm_reg_count,
      used_spill_slot_halfs: [0u64; 8],
      max_used_slot: 0,
      next_spill_id: 1,
      spills: Vec::new(),
      exit_sync_args: DenseHashMap::new(u32::MAX),
      alloc_action_count: 0,
    }
  }
}
