use alloc::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, small_vector::SmallVector};

use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, exit_sync_arg_a_64::ExitSyncArgA64,
  ir_function::IrFunction, lowering_stats::LoweringStats, set::Set, spill::Spill,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrRegAllocA64 {
  pub build: *mut AssemblyBuilderA64,
  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,

  pub curr_inst_idx: u32,

  pub gpr: Set,
  pub simd: Set,

  pub spills: Vec<Spill>,

  pub free_spill_slots: u64,

  pub exit_sync_args: DenseHashMap<u32, SmallVector<ExitSyncArgA64, 2>>,

  pub alloc_action_count: u32,

  pub error: bool,
}

impl IrRegAllocA64 {
  pub(crate) const K_INVALID_INST_IDX: u32 = 0xFFFFFFFF;
}
