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

use alloc::string::String;
use core::ptr::null_mut;

use ulua_code_gen::enums::abix_64::ABIX64;

impl IrRegAllocX64Fixture {
  pub fn new() -> Self {
    let mut build = Box::new(AssemblyBuilderX64::new_with_abi(true, ABIX64::Windows, 0));
    let mut function = Box::new(IrFunction::default());
    let regs = IrRegAllocX64::new(build.as_mut(), function.as_mut(), null_mut());

    Self {
      build,
      function,
      regs,
    }
  }

  pub fn check_match(&mut self, expected: String) {
    self.build.finalize();
    assert_eq!(alloc::format!("\n{}", self.build.text), expected);
  }
}

impl Default for IrRegAllocX64Fixture {
  fn default() -> Self {
    Self::new()
  }
}
