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

use alloc::string::String;

use ulua_code_gen::{
  enums::{abix_64::ABIX64, size_x_64::SizeX64},
  records::{
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, operand_x_64::OperandX64,
    scoped_reg_x_64::ScopedRegX64,
  },
};

impl IrCallWrapperX64Fixture {
  pub fn new(abi: ABIX64) -> Self {
    let mut build = Box::new(AssemblyBuilderX64::new_with_abi(true, abi, 0));
    let mut function = Box::new(IrFunction::default());
    let mut regs = Box::new(IrRegAllocX64::new(&mut build, &mut function, None));
    let call_wrap = IrCallWrapperX64::new(&mut regs, &mut build, !0u32);

    Self {
      build,
      function,
      regs,
      call_wrap,
      r_arg1: RegisterX64::RCX,
      r_arg1d: RegisterX64::ECX,
      r_arg2: RegisterX64::RDX,
      r_arg2d: RegisterX64::EDX,
      r_arg3: RegisterX64::R8,
      r_arg3d: RegisterX64::R8D,
      r_arg4: RegisterX64::R9,
      r_arg4d: RegisterX64::R9D,
    }
  }

  pub fn windows() -> Self {
    Self::new(ABIX64::Windows)
  }

  pub fn take_scoped(&mut self, reg: RegisterX64) -> ScopedRegX64 {
    let reg = self.regs.take_reg(reg, K_INVALID_INST_IDX);
    ScopedRegX64 {
      owner: &mut *self.regs,
      reg,
    }
  }

  pub fn add_arg(&mut self, target_size: SizeX64, source: impl Into<OperandX64>) {
    self
      .call_wrap
      .add_argument_op(target_size, source.into(), IrOp::new());
  }

  pub fn add_scoped(&mut self, target_size: SizeX64, scoped_reg: &mut ScopedRegX64) {
    self.call_wrap.add_argument_reg(target_size, scoped_reg);
  }

  pub fn call(&mut self, func: OperandX64) {
    self.call_wrap.call(&func);
  }

  pub fn check_match(&mut self, expected: String) {
    self.regs.assert_all_free();
    self.build.finalize();
    assert_eq!(alloc::format!("\n{}", self.build.text), expected);
  }
}
