use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, exit_handler_ir_lowering_a_64::ExitHandler,
  interrupt_handler_ir_lowering_a_64::InterruptHandler, ir_function::IrFunction,
  ir_reg_alloc_a_64::IrRegAllocA64, ir_value_location_tracking::IrValueLocationTracking,
  lowering_stats::LoweringStats, module_helpers::ModuleHelpers,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrLoweringA64 {
  pub build: *mut AssemblyBuilderA64,
  pub helpers: *mut ModuleHelpers,
  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,
  pub regs: IrRegAllocA64,
  pub value_tracker: IrValueLocationTracking,
  pub interrupt_handlers: Vec<InterruptHandler>,
  pub exit_handlers: Vec<ExitHandler>,
  pub exit_handler_map: DenseHashMap<u32, u32>,
  pub exit_sync_alloc_token: u32,
  pub exit_sync_inst_idx: u32,
  pub error: bool,
}

impl IrLoweringA64 {
  pub const K_INVALID_INST_IDX: u32 = !0u32;
}
