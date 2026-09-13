use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{substitute::substitute, vm_upvalue_op::vm_upvalue_op},
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
};

impl ConstPropState {
  pub fn substitute_or_record_vm_upvalue_load(&mut self, load_inst: &mut IrInst) -> bool {
    CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmUpvalue);

    let key = vm_upvalue_op(op_a(load_inst));
    let key_u8 = key as u8;
    if let Some(prev_idx) = self.upvalue_map.find(&key_u8)
      && *prev_idx != u32::MAX
    {
      // Substitute load instruction with the previous value
      unsafe {
        substitute(
          &mut *self.function,
          load_inst,
          IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, *prev_idx),
        );
      }
      return true;
    }

    let inst_idx = unsafe { (*self.function).get_inst_index(load_inst) };

    // Record load of this upvalue for future substitution
    *self.upvalue_map.get_or_insert(key_u8) = inst_idx;
    false
  }
}
