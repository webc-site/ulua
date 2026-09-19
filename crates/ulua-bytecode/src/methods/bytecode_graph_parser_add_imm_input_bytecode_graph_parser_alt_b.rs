use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_inst::BcInst,
    bc_op::BcOp,
    bytecode_graph_parser::BytecodeGraphParser,
  },
};

impl<'a> BytecodeGraphParser<'a> {
  /// # Safety
  ///
  /// `inst` must be a valid, aligned, non-null pointer to a `BcInst`.
  /// cpp `addImmInput(inst, int32_t)`：每个立即数操作数独占一条 `immediates` 记录，
  /// 不按值复用——同一指令的 paramCount / returnCount / fbSlot 可能取值相同，复用会让
  /// 后续 `setFbSlot` 之类的改写连带改掉另一个操作数。
  pub unsafe fn add_imm_input_bc_inst_i32(&mut self, inst: *mut BcInst, value: i32) {
    let inst = unsafe { &mut *inst };
    self.func.immediates.push(BcImm {
      kind: BcImmKind::Int,
      value: BcImmValue { value_int: value },
    });
    let op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.func.immediates.len() - 1) as u32);
    inst.ops.push_back(op);
  }
}
