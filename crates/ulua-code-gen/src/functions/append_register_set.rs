use core::{fmt::Write, mem::size_of};

use crate::records::{ir_to_string_context::IrToStringContext, register_set::RegisterSet};

/// `RegisterSet::regs` 为 4 个 u64，共 256 个寄存器位
const REGISTER_SET_BITS: usize = size_of::<[u64; 4]>() * 8;
const BITS_PER_WORD: usize = u64::BITS as usize;

pub fn append_register_set(ctx: &mut IrToStringContext, rs: &RegisterSet, separator: &str) {
  let mut comma = false;

  // 只扫描置位位（等价于逐位遍历，输出顺序仍按寄存器号升序）
  debug_assert_eq!(REGISTER_SET_BITS, rs.regs.len() * BITS_PER_WORD);
  for (word_idx, &regs) in rs.regs.iter().enumerate() {
    let mut bits = regs;
    while bits != 0 {
      let bit_idx = bits.trailing_zeros() as usize;
      bits &= bits - 1; // 清除最低置位位

      if comma {
        ctx.result.push_str(separator);
      }
      comma = true;

      let _ = write!(ctx.result, "R{}", word_idx * BITS_PER_WORD + bit_idx);
    }
  }

  if rs.vararg_seq {
    if comma {
      ctx.result.push_str(separator);
    }

    let _ = write!(ctx.result, "R{}...", rs.vararg_start);
  }
}
