use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, exit_handler_ir_lowering_x_64::ExitHandler,
  interrupt_handler_ir_lowering_x_64::InterruptHandler, ir_function::IrFunction,
  ir_reg_alloc_x_64::IrRegAllocX64, ir_value_location_tracking::IrValueLocationTracking,
  lowering_stats::LoweringStats, module_helpers::ModuleHelpers, operand_x_64::OperandX64,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrLoweringX64 {
  pub build: *mut AssemblyBuilderX64,
  pub helpers: *mut ModuleHelpers,

  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,

  pub regs: IrRegAllocX64,

  pub value_tracker: IrValueLocationTracking,

  pub interrupt_handlers: Vec<InterruptHandler>,
  pub exit_handlers: Vec<ExitHandler>,
  pub exit_handler_map: DenseHashMap<u32, u32>,

  pub vector_and_mask: OperandX64,
  pub vector_or_mask: OperandX64,

  pub exit_sync_alloc_token: u32,
  pub exit_sync_inst_idx: u32,
}

impl IrLoweringX64 {
  pub(crate) const K_INVALID_INST_IDX: u32 = !0u32;
}
