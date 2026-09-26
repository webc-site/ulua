use crate::{
  functions::reg_bitset::{reg_bit_set, reg_bit_test},
  records::register_set::RegisterSet,
};

pub fn require_variadic_sequence(
  source_rs: &mut RegisterSet,
  def_rs: &RegisterSet,
  mut vararg_start: u8,
) {
  if !def_rs.vararg_seq {
    // 从 variadic 序列中剥离由本指令定义的寄存器
    while reg_bit_test(&def_rs.regs, vararg_start as usize) {
      vararg_start = vararg_start.wrapping_add(1);
    }

    // 若该 use 更早已被 require，sourceRs.varargSeq 可能已为 true。
    // 断言起点一致。
    if source_rs.vararg_seq {
      assert!(source_rs.vararg_start == vararg_start);
    }

    source_rs.vararg_seq = true;
    source_rs.vararg_start = vararg_start;
  } else {
    // variadic use 序列可能包含位于 def 序列之前的寄存器
    for i in vararg_start..def_rs.vararg_start {
      if !reg_bit_test(&def_rs.regs, i as usize) {
        reg_bit_set(&mut source_rs.regs, i as usize);
      }
    }
  }
}
