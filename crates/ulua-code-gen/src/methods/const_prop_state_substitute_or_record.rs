use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{has_side_effects::has_side_effects, substitute::substitute},
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
};

impl ConstPropState {
  pub fn substitute_or_record(&mut self, inst: &mut IrInst, inst_idx: u32) {
    if let Some(prev_idx) = self.value_map.find(inst).copied() {
      let prev_is_valid = unsafe {
        let prev = &(&(*self.function).instructions)[prev_idx as usize];
        prev.use_count != 0 || has_side_effects(prev.cmd)
      };

      if prev_is_valid {
        unsafe {
          substitute(
            &mut *self.function,
            inst,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
          );
        }
        return;
      }
    }

    *self.value_map.get_or_insert(inst.clone()) = inst_idx;
  }
}
