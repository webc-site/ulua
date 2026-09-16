use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, ir_function::IrFunction,
    ir_reg_alloc_a_64::IrRegAllocA64, lowering_stats::LoweringStats, register_a_64::RegisterA64,
    set::Set,
  },
};

impl IrRegAllocA64 {
  pub fn ir_reg_alloc_a_64_ir_reg_alloc_a_64(
    build: &mut AssemblyBuilderA64,
    function: &mut IrFunction,
    stats: *mut LoweringStats,
    regs: &[(RegisterA64, RegisterA64)],
  ) -> Self {
    let mut alloc = IrRegAllocA64 {
      build: build as *mut AssemblyBuilderA64,
      function: function as *mut IrFunction,
      stats,
      curr_inst_idx: IrRegAllocA64::K_INVALID_INST_IDX,
      gpr: Set {
        base: 0,
        free: 0,
        temp: 0,
        defs: [u32::MAX; 32],
      },
      simd: Set {
        base: 0,
        free: 0,
        temp: 0,
        defs: [u32::MAX; 32],
      },
      spills: Vec::new(),
      free_spill_slots: 0,
      exit_sync_args: DenseHashMap::new(!0u32),
      alloc_action_count: 0,
      error: false,
    };

    for (first, second) in regs {
      CODEGEN_ASSERT!(first.kind() == second.kind() && first.index() <= second.index());

      let set = alloc.get_set(first.kind());
      for i in first.index()..=second.index() {
        set.base |= 1u32 << i;
      }
    }

    alloc.gpr.free = alloc.gpr.base;
    alloc.simd.free = alloc.simd.base;

    const K_SPILL_SLOTS: u32 = 16;
    const K_EXTRA_SPILL_SLOTS: u32 = 8;
    CODEGEN_ASSERT!(K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS < 64);
    alloc.free_spill_slots = (1u64 << (K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS)) - 1u64;

    alloc
  }
}
