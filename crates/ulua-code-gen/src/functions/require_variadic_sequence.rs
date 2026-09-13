use crate::records::register_set::RegisterSet;

pub fn require_variadic_sequence(
  source_rs: &mut RegisterSet,
  def_rs: &RegisterSet,
  mut vararg_start: u8,
) {
  if !def_rs.vararg_seq {
    // Peel away registers from variadic sequence that we define
    while def_rs.regs[vararg_start as usize / 64] & (1u64 << (vararg_start as usize % 64)) != 0 {
      vararg_start = vararg_start.wrapping_add(1);
    }

    // sourceRs.varargSeq might already be true if the use was required earlier.
    // Assert the start is consistent.
    if source_rs.vararg_seq {
      assert!(source_rs.vararg_start == vararg_start);
    }

    source_rs.vararg_seq = true;
    source_rs.vararg_start = vararg_start;
  } else {
    // Variadic use sequence might include registers before def sequence
    for i in vararg_start..def_rs.vararg_start {
      let word_idx = (i as usize) / 64;
      let bit_idx = (i as usize) % 64;

      if def_rs.regs[word_idx] & (1u64 << bit_idx) == 0 {
        source_rs.regs[word_idx] |= 1u64 << bit_idx;
      }
    }
  }
}
