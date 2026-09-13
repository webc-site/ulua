use core::mem::size_of;

use crate::records::{ir_to_string_context::IrToStringContext, register_set::RegisterSet};

/// `RegisterSet::regs` 为 4 个 u64，共 256 个寄存器位
const REGISTER_SET_BITS: usize = size_of::<[u64; 4]>() * 8;

pub fn append_register_set(ctx: &mut IrToStringContext, rs: &RegisterSet, separator: &str) {
  let mut comma = false;

  for i in 0..REGISTER_SET_BITS {
    let word_idx = i / 64;
    let bit_idx = i % 64;

    if (rs.regs[word_idx] & (1 << bit_idx)) != 0 {
      if comma {
        ctx.result.push_str(separator);
      }
      comma = true;

      use core::fmt::Write;
      let _ = write!(ctx.result, "R{}", i);
    }
  }

  if rs.vararg_seq {
    if comma {
      ctx.result.push_str(separator);
    }

    use core::fmt::Write;
    let _ = write!(ctx.result, "R{}...", rs.vararg_start);
  }
}
