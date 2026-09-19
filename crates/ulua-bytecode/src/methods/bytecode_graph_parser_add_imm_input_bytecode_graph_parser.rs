use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_op::BcOp,
    bytecode_graph_parser::BytecodeGraphParser,
  },
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addImmInput(BcRef<BcInst>, bool)`（`BytecodeGraphParser.h:402-409`）：
  /// 无条件 `push_back` 一条新的 Boolean 立即数，索引取 `size - 1`，**不按值复用**
  /// （与 `int32_t`/`uint32_t` 两版一致；此前的 `position()` 去重会让同一指令的多个
  /// bool 操作数共享同一条 `immediates` 记录，改写其一时连带污染另一个）。
  pub fn add_imm_input_bc_inst_bool(&mut self, inst: BcOp, value: bool) {
    self.func.immediates.push(BcImm {
      kind: BcImmKind::Boolean,
      value: BcImmValue {
        value_boolean: value,
      },
    });

    let op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.func.immediates.len() - 1) as u32);
    self.func.inst_op(inst).ops.push_back(op);
  }
}
