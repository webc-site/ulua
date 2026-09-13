use alloc::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, small_vector::SmallVector};

use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, exit_sync_arg_x_64::ExitSyncArgX64,
  ir_function::IrFunction, ir_spill_x_64::IrSpillX64, lowering_stats::LoweringStats,
  register_x_64::RegisterX64,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrRegAllocX64 {
  pub build: *mut AssemblyBuilderX64,
  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,

  pub curr_inst_idx: u32,

  pub free_gpr_map: [bool; 16],
  pub gpr_inst_users: [u32; 16],
  pub free_xmm_map: [bool; 16],
  pub xmm_inst_users: [u32; 16],
  pub usable_xmm_reg_count: u8,

  pub used_spill_slot_halfs: [u64; 8], // std::bitset<512> mapped to fixed-size array
  pub max_used_slot: u32,

  pub next_spill_id: u32,
  pub spills: Vec<IrSpillX64>,

  pub exit_sync_args: DenseHashMap<u32, SmallVector<ExitSyncArgX64, 2>>,

  pub alloc_action_count: u32,
}

impl IrRegAllocX64 {
  /// `static const RegisterX64 kGprAllocOrder[] = {rax, rdx, rcx, rbx, rsi, rdi, r8, r9, r10, r11};`
  /// (IrRegAllocX64.cpp:23)
  pub const K_GPR_ALLOC_ORDER: [RegisterX64; 10] = [
    RegisterX64::RAX,
    RegisterX64::RDX,
    RegisterX64::RCX,
    RegisterX64::RBX,
    RegisterX64::RSI,
    RegisterX64::RDI,
    RegisterX64::R8,
    RegisterX64::R9,
    RegisterX64::R10,
    RegisterX64::R11,
  ];
}
