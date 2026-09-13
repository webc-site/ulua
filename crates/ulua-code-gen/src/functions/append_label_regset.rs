use crate::{
  functions::{append::append, append_register_set::append_register_set},
  records::{ir_to_string_context::IrToStringContext, register_set::RegisterSet},
};

pub fn append_label_regset(
  ctx: &mut IrToStringContext,
  reg_sets: &[RegisterSet],
  block_idx: usize,
  name: &str,
) {
  if block_idx < reg_sets.len() {
    let rs = &reg_sets[block_idx];

    if rs.regs.iter().any(|&r| r != 0) || rs.vararg_seq {
      append(ctx.result, format_args!("|{{{}|", name));
      append_register_set(ctx, rs, "|");
      append(ctx.result, format_args!("}}"));
    }
  }
}
