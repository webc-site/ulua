use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{
    ir_builder::IrBuilder,
    ir_const::{IrConst, IrConstValue},
    ir_op::IrOp,
  },
};
impl IrBuilder {
  pub fn const_import(&mut self, value: u32) -> IrOp {
    let constant = IrConst {
      kind: IrConstKind::Import,
      value: IrConstValue { value_uint: value },
    };

    self.const_any(constant, value as u64)
  }
}
