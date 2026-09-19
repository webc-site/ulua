use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{
    ir_builder::{ConstantKey, IrBuilder},
    ir_const::IrConst,
    ir_op::IrOp,
  },
};

impl IrBuilder {
  pub fn const_any(&mut self, constant: IrConst, as_common_key: u64) -> IrOp {
    let key = ConstantKey {
      kind: constant.kind,
      value: as_common_key,
    };

    if let Some(&index) = self.constant_map.find(&key) {
      return IrOp::ir_op_ir_op_kind_u32(IrOpKind::Constant, index);
    }

    let index = self.function.constants.len() as u32;
    self.function.constants.push(constant);
    *self.constant_map.get_or_insert(key) = index;

    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Constant, index)
  }
}
