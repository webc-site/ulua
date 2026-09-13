use crate::{
  functions::vm_upvalue_op::vm_upvalue_op,
  macros::{op_a::op_a, op_b::op_b},
  records::{const_prop_state::ConstPropState, ir_inst::IrInst},
};

impl ConstPropState {
  pub fn forward_vm_upvalue_store_to_load(&mut self, store_inst: &mut IrInst) {
    let upvalue_index = vm_upvalue_op(op_a(store_inst)) as u8;
    let value_index = op_b(store_inst.clone()).index();
    *self.upvalue_map.get_or_insert(upvalue_index) = value_index;
  }
}
