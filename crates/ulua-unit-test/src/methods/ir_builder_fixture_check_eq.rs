use ulua_code_gen::records::{ir_inst::IrInst, ir_inst_eq::IrInstEq, ir_op::IrOp};

use crate::records::ir_builder_fixture::IrBuilderFixture;

impl IrBuilderFixture {
  /// Port of C++ `IrBuilderFixture::checkEq`. Looks up the instruction at
  /// `inst_op` in the function and asserts it equals `inst` under `IrInstEq`
  /// (command + operand) comparison.
  pub fn check_eq(&mut self, inst_op: IrOp, inst: &IrInst) {
    let target = self.build.function.inst_op(inst_op);
    let inst_eq = IrInstEq;
    assert!(inst_eq.ir_inst_eq_operator_call(target, inst));
  }
}
