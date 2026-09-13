use alloc::boxed::Box;

use ulua_code_gen::records::{
  assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
  ir_function::IrFunction, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64,
};

#[derive(Debug)]
pub struct IrCallWrapperX64Fixture {
  pub build: Box<AssemblyBuilderX64>,
  pub function: Box<IrFunction>,
  pub regs: Box<IrRegAllocX64>,
  pub call_wrap: IrCallWrapperX64,

  // Tests rely on these to force interference between registers
  pub r_arg1: RegisterX64,
  pub r_arg1d: RegisterX64,
  pub r_arg2: RegisterX64,
  pub r_arg2d: RegisterX64,
  pub r_arg3: RegisterX64,
  pub r_arg3d: RegisterX64,
  pub r_arg4: RegisterX64,
  pub r_arg4d: RegisterX64,
}
