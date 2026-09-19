use crate::{
  enums::ir_op_kind::IrOpKind, functions::any_argument_match::any_argument_match,
  records::remove_dead_store_state::RemoveDeadStoreState,
};
impl RemoveDeadStoreState {
  pub fn has_remaining_uses(&mut self, inst_idx: u32) -> bool {
    let inst = unsafe { &*self.function }
      .instructions
      .get(inst_idx as usize);
    match inst {
      Some(inst) => any_argument_match(inst, |op| {
        op.kind() == IrOpKind::Inst && unsafe { &*self.remaining_uses }[op.index() as usize] != 0
      }),
      None => false,
    }
  }
}
