use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{replace_ir_utils::replace_ir_function_ir_op_ir_op, vm_reg_op::vm_reg_op},
  macros::op_a::op_a,
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
};

impl ConstPropState {
  pub fn try_redirect_vm_reg_load_to_t_value_origin(&mut self, load_inst: &mut IrInst) -> bool {
    let source = op_a(load_inst);

    if let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadTvalue, source) {
      let prev_idx = unsafe { *prev_idx };
      let tvalue_load = unsafe { &(&(*self.function).instructions)[prev_idx as usize] };
      let tvalue_source = op_a(&mut tvalue_load.clone());

      if tvalue_load.cmd != IrCmd::LoadTvalue || tvalue_source.kind() != IrOpKind::VmReg {
        return false;
      }

      if vm_reg_op(tvalue_source) == vm_reg_op(source) {
        return false;
      }

      let prev_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx);
      if self.try_get_reg_link(prev_op).is_none() {
        return false;
      }

      unsafe {
        replace_ir_function_ir_op_ir_op(&mut *self.function, &mut load_inst.ops[0], tvalue_source);
      }
      return true;
    }

    false
  }
}
