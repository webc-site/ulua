use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    get_cmd_value_kind::get_cmd_value_kind, get_const_value_kind::get_const_value_kind,
    substitute::substitute,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{
    const_prop_state::ConstPropState, ir_builder::IrBuilder, ir_inst::IrInst, ir_op::IrOp,
  },
};

impl ConstPropState {
  pub fn substitute_or_record_value_load_with_t_value_data(
    &mut self,
    _build: &mut IrBuilder,
    load_inst: &mut IrInst,
  ) -> bool {
    CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmReg);

    if let Some(prev_idx) =
      self.get_previous_versioned_load_index(IrCmd::LoadTvalue, op_a(load_inst))
    {
      if let Some(value_op) = self.inst_value.find(&unsafe { *prev_idx }) {
        let value_ptr = unsafe { (*self.function).as_inst_op(*value_op) };
        if !value_ptr.is_null() {
          let value = unsafe { &*value_ptr };
          if value.use_count != 0 && value.cmd == load_inst.cmd {
            unsafe { substitute(&mut *self.function, load_inst, *value_op) };
            return true;
          }

          if value.use_count != 0
            && get_cmd_value_kind(value.cmd) == get_cmd_value_kind(load_inst.cmd)
          {
            unsafe { substitute(&mut *self.function, load_inst, *value_op) };
            return true;
          }
        } else if value_op.kind() == IrOpKind::Constant {
          let constant = unsafe { (*self.function).const_op(*value_op) };
          if get_const_value_kind(&constant) == get_cmd_value_kind(load_inst.cmd) {
            unsafe { substitute(&mut *self.function, load_inst, *value_op) };
            return true;
          }
        }
      } else {
        let idx = unsafe { (*self.function).get_inst_index(load_inst) };
        self.inst_value.try_insert(
          unsafe { *prev_idx },
          IrOp::ir_op_kind_u32(IrOpKind::Inst, idx),
        );
      }
    }

    false
  }
}
