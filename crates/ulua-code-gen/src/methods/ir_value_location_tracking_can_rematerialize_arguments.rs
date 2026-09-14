use crate::{
  enums::ir_op_kind::IrOpKind,
  macros::op_a::op_a,
  records::{ir_inst::IrInst, ir_value_location_tracking::IrValueLocationTracking},
};

impl IrValueLocationTracking {
  pub fn can_rematerialize_arguments(&mut self, inst: &mut IrInst) -> bool {
    if self.can_be_rematerialized(inst.cmd) && op_a(inst).kind() == IrOpKind::Inst {
      let function = unsafe { &mut *self.function };
      let dep_inst = function.inst_op(op_a(inst));

      if dep_inst.last_use != function.get_inst_index(inst) {
        return true;
      }
    }

    false
  }
}
