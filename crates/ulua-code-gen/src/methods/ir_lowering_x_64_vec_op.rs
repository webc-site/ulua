use crate::{
  enums::{ir_cmd::IrCmd, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

impl IrLoweringX64 {
  pub fn vec_op(&mut self, op: IrOp, tmp: &mut ScopedRegX64) -> RegisterX64 {
    let function = self.function;
    let source = unsafe { (*function).inst_op(op) };

    CODEGEN_ASSERT!(source.cmd != IrCmd::SUBSTITUTE);

    if source.cmd != IrCmd::LoadTvalue
      && source.cmd != IrCmd::GetUpvalue
      && source.cmd != IrCmd::TagVector
    {
      return self.reg_op(op);
    }

    tmp.alloc(SizeX64::Xmmword);
    let dst = OperandX64::reg(tmp.reg);
    let src1 = OperandX64::reg(self.reg_op(op));
    let src2 = self.vector_and_mask_op();
    unsafe { (*self.build).vandps(dst, src1, src2) };
    tmp.reg
  }
}
