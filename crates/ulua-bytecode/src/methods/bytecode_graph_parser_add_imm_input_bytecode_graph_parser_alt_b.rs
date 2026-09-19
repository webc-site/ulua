use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_op::BcOp,
    bytecode_graph_parser::BytecodeGraphParser,
  },
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addImmInput(BcRef<BcInst>, int32_t)`（`BytecodeGraphParser.h:411-418`）：
  /// 每个立即数操作数独占一条 `immediates` 记录，不按值复用——同一指令的
  /// paramCount / returnCount / fbSlot 可能取值相同，复用会让后续 `setFbSlot`
  /// 之类的改写连带改掉另一个操作数。
  pub fn add_imm_input_bc_inst_i32(&mut self, inst: BcOp, value: i32) {
    self.func.immediates.push(BcImm {
      kind: BcImmKind::Int,
      value: BcImmValue { value_int: value },
    });

    let op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.func.immediates.len() - 1) as u32);
    self.func.inst_op(inst).ops.push_back(op);
  }
}
