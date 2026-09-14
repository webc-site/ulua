use crate::{
  functions::{
    kill_ir_utils::kill_ir_function_ir_inst, replace_ir_utils::replace_ir_function_ir_op_ir_op,
  },
  macros::{op_c::op_c, op_d::op_d},
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_builder::IrBuilder, ir_inst::IrInst,
  },
};

impl ConstPropState {
  pub fn try_merge_and_kill_buffer_length_check(
    &mut self,
    build: &mut IrBuilder,
    _block: &mut IrBlock,
    curr_check: &mut IrInst,
    prev_check: &mut IrInst,
    extra_offset: i32,
  ) -> bool {
    let prev_min_offset = unsafe { (*self.function).int_op(op_c(prev_check.clone())) };
    let prev_max_offset = unsafe { (*self.function).int_op(op_d(prev_check.clone())) };
    let curr_min_offset =
      unsafe { (*self.function).int_op(op_c(curr_check.clone())) } + extra_offset;
    let curr_max_offset =
      unsafe { (*self.function).int_op(op_d(curr_check.clone())) } + extra_offset;
    let new_min_offset = prev_min_offset.min(curr_min_offset);
    let new_max_offset = prev_max_offset.max(curr_max_offset);

    if new_max_offset - new_min_offset > 4095 {
      return false;
    }
    if !(-4095..=4095).contains(&new_min_offset) {
      return false;
    }

    if new_min_offset != prev_min_offset {
      let replacement = build.const_int(new_min_offset);
      unsafe {
        replace_ir_function_ir_op_ir_op(&mut *self.function, &mut prev_check.ops[2], replacement);
      }
    }

    if new_max_offset != prev_max_offset {
      let replacement = build.const_int(new_max_offset);
      unsafe {
        replace_ir_function_ir_op_ir_op(&mut *self.function, &mut prev_check.ops[3], replacement);
      }
    }

    unsafe { kill_ir_function_ir_inst(&mut *self.function, curr_check) };
    true
  }
}
