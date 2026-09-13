use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, call_argument::CallArgument,
  ir_call_wrapper_x_64::IrCallWrapperX64, ir_reg_alloc_x_64::IrRegAllocX64,
  operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl IrCallWrapperX64 {
  pub fn ir_call_wrapper_x_64_ir_call_wrapper_x_64(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    inst_idx: u32,
  ) -> Self {
    let mut wrapper = Self {
      regs: regs as *mut IrRegAllocX64,
      build: build as *mut AssemblyBuilderX64,
      inst_idx,
      args: [
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
      ],
      arg_count: 0,
      gpr_pos: 0,
      xmm_pos: 0,
      func_op: OperandX64::operand_x_64_register_x_64(RegisterX64::NOREG),
      result_reg: RegisterX64::NOREG,
      result_inst_idx: 0,
      gpr_uses: [0u8; 16],
      xmm_uses: [0u8; 16],
    };

    wrapper.gpr_uses.fill(0);
    wrapper.xmm_uses.fill(0);

    wrapper
  }
}
