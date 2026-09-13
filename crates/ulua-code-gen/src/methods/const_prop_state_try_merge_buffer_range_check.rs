use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::{
    get_cmd_value_kind::get_cmd_value_kind, replace_ir_utils::replace_ir_function_ir_op_ir_op,
    replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst,
  },
  macros::{op_a::op_a, op_b::op_b, op_e::op_e},
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_builder::IrBuilder, ir_inst::IrInst,
  },
  type_aliases::ir_ops::IrOps,
};
impl ConstPropState {
  pub fn try_merge_buffer_range_check(
    &mut self,
    build: &mut IrBuilder,
    block: &mut IrBlock,
    inst: &mut IrInst,
    prev: &mut IrInst,
  ) -> bool {
    if op_a(inst) != op_a(prev) {
      return false;
    }

    let curr_index_op = op_b(inst.clone());
    let prev_index_op = op_b(prev.clone());
    let curr_index = unsafe { (*self.function).as_inst_op(curr_index_op) };
    let prev_index = unsafe { (*self.function).as_inst_op(prev_index_op) };
    if curr_index.is_null() || prev_index.is_null() {
      return false;
    }

    let curr_index = unsafe { &*curr_index };
    let prev_index = unsafe { &*prev_index };

    if curr_index.cmd == IrCmd::NumToInt && prev_index.cmd == IrCmd::NumToInt {
      let curr_index_source = op_a(&mut curr_index.clone());
      let prev_index_source = op_a(&mut prev_index.clone());
      let offset_base_curr = self.get_offset_base(curr_index_source);
      let offset_base_prev = self.get_offset_base(prev_index_source);
      if offset_base_curr.op == offset_base_prev.op
        && offset_base_curr.scale == offset_base_prev.scale
        && offset_base_curr.op.kind() != IrOpKind::Constant
      {
        let extra_offset = offset_base_curr.offset - offset_base_prev.offset;
        if extra_offset != 0 {
          if prev_index_op.index() >= curr_index_op.index() {
            return false;
          }
          let offset = build.const_int(extra_offset);
          let mut ops = IrOps::new();
          ops.push(prev_index_op);
          ops.push(offset);
          unsafe {
            replace_ir_function_ir_block_u32_ir_inst(
              &mut *self.function,
              block,
              curr_index_op.index(),
              IrInst {
                cmd: IrCmd::AddInt,
                ops,
                ..IrInst::default()
              },
            );
          }
        }

        if op_e(prev.clone()).kind() == IrOpKind::Undef {
          let replacement = op_a(&mut prev_index.clone());
          unsafe {
            replace_ir_function_ir_op_ir_op(&mut *self.function, &mut prev.ops[4], replacement);
          }
        }
        return self.try_merge_and_kill_buffer_length_check(build, block, inst, prev, extra_offset);
      }
    } else if get_cmd_value_kind(curr_index.cmd) == IrValueKind::Int
      && get_cmd_value_kind(prev_index.cmd) == IrValueKind::Int
    {
      let offset_base_curr = self.get_offset_base(curr_index_op);
      let offset_base_prev = self.get_offset_base(prev_index_op);
      if offset_base_curr.op == offset_base_prev.op
        && offset_base_curr.scale == offset_base_prev.scale
      {
        let extra_offset = offset_base_curr.offset - offset_base_prev.offset;
        return self.try_merge_and_kill_buffer_length_check(build, block, inst, prev, extra_offset);
      }
    }
    false
  }
}
