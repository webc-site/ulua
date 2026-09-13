use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, call_argument::CallArgument,
  ir_reg_alloc_x_64::IrRegAllocX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrCallWrapperX64 {
  pub regs: *mut IrRegAllocX64,
  pub build: *mut AssemblyBuilderX64,
  pub inst_idx: u32,
  pub(crate) args: [CallArgument; 6],
  pub(crate) arg_count: i32,
  pub(crate) gpr_pos: i32,
  pub(crate) xmm_pos: i32,
  pub(crate) func_op: OperandX64,
  pub(crate) result_reg: RegisterX64,
  pub(crate) result_inst_idx: u32,
  pub(crate) gpr_uses: [u8; 16],
  pub(crate) xmm_uses: [u8; 16],
}

impl IrCallWrapperX64 {
  pub(crate) const K_MAX_CALL_ARGUMENTS: i32 = 6;
}
