use alloc::boxed::Box;

use ulua_code_gen::records::{
  assembly_builder_x_64::AssemblyBuilderX64, ir_function::IrFunction,
  ir_reg_alloc_x_64::IrRegAllocX64,
};

#[derive(Debug)]
pub struct IrRegAllocX64Fixture {
  pub build: Box<AssemblyBuilderX64>,
  pub function: Box<IrFunction>,
  pub regs: IrRegAllocX64,
}
